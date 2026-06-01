# Runtime architecture

До сих пор мы говорили об async runtimes как об абстрактной концепции.
Давайте подробнее разберём их реализацию: вскоре вы увидите, что она влияет на наш код.

## Flavors

`tokio` предоставляет два разных _flavors_ runtime.

Настроить runtime можно через `tokio::runtime::Builder`:

- `Builder::new_multi_thread` предоставляет **multithreaded `tokio` runtime**
- `Builder::new_current_thread` использует для выполнения **current thread**.

`#[tokio::main]` по умолчанию возвращает multithreaded runtime, а
`#[tokio::test]` использует current thread runtime.

### Current thread runtime

Current-thread runtime, как следует из названия, для scheduling и выполнения tasks
использует исключительно OS thread, в котором был запущен.\
При использовании current-thread runtime доступна **concurrency**, но не **parallelism**:
asynchronous tasks чередуются, однако в каждый момент времени выполняется не более одного task.

### Multithreaded runtime

При использовании multithreaded runtime в каждый момент времени _in parallel_
может выполняться до `N` tasks, где `N` — число threads, используемых runtime.
По умолчанию `N` совпадает с числом доступных CPU cores.

Более того, `tokio` применяет **work-stealing**.\
Если thread бездействует, он не ждёт, а пытается найти готовый к выполнению task:
в global queue или в local queue другого thread.\
Work-stealing может заметно повысить производительность, особенно tail latencies,
если workload приложения распределён между threads неравномерно.

## Implications

`tokio::spawn` не зависит от flavor: он работает как с multithreaded, так и с
current-thread runtime. Недостаток в том, что signature исходит из худшего случая
(то есть multithreaded) и содержит соответствующие bounds:

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{ /* */ }
```

Пока проигнорируем trait `Future`, чтобы сосредоточиться на остальном.\
`spawn` требует, чтобы все входные values были `Send` и имели lifetime `'static`.

Constraint `'static` обусловлен той же причиной, что и у `std::thread::spawn`:
spawned task может пережить породивший его context, поэтому не должен зависеть
от local data, которые могут быть освобождены после уничтожения этого context.

```rust
fn spawner() {
    let v = vec![1, 2, 3];
    // Это не сработает, поскольку `&v`
    // живет недостаточно долго.
    tokio::spawn(async { 
        for x in &v {
            println!("{x}")
        }
    })
}
```

Bound `Send`, в свою очередь, напрямую следует из стратегии work-stealing в `tokio`:
task, созданный в thread `A`, может быть перемещён в бездействующий thread `B`.
Поскольку пересекается thread boundary, необходим bound `Send`.

```rust
fn spawner(input: Rc<u64>) {
    // Это тоже не сработает, поскольку
    // `Rc` не реализует `Send`.
    tokio::spawn(async move {
        println!("{}", input);
    })
}
```
