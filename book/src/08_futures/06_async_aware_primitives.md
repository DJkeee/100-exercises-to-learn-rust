# Async-aware primitives

В documentation `tokio` можно заметить множество types, которые соответствуют types
из standard library, но адаптированы для asynchronous context: locks, channels,
timers и другие.

При работе в asynchronous context следует отдавать предпочтение этим asynchronous alternatives,
а не их synchronous-аналогам.

Чтобы понять причину, рассмотрим `Mutex` — mutually exclusive lock,
изученный в предыдущей главе.

## Case study: `Mutex`

Рассмотрим простой пример:

```rust
use std::sync::{Arc, Mutex};

async fn run(m: Arc<Mutex<Vec<u64>>>) {
    let guard = m.lock().unwrap();
    http_call(&guard).await;
    println!("Sent {:?} to the server", &guard);
    // Здесь удаляется `guard`
}

/// Использует `v` в качестве тела HTTP-вызова.
async fn http_call(v: &[u64]) {
  // [...]
}
```

### `std::sync::MutexGuard` and yield points

Этот код компилируется, но опасен.

Мы пытаемся захватить lock для `Mutex` из `std` в asynchronous context.
Затем сохраняем полученный `MutexGuard` при прохождении через yield point
(`.await` для `http_call`).

Представим, что два tasks concurrently выполняют `run` в single-threaded runtime.
Получится следующая последовательность scheduling events:

```text
     Task A          Task B
        | 
  Acquire lock
Yields to runtime
        | 
        +--------------+
                       |
             Tries to acquire lock
```

Возникает deadlock. Task B никогда не захватит lock, поскольку lock удерживается task A,
который передал управление runtime до освобождения lock. Task A не будет назначен снова,
поскольку runtime не может preempt task B.

### `tokio::sync::Mutex`

Проблему можно решить переходом на `tokio::sync::Mutex`:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

async fn run(m: Arc<Mutex<Vec<u64>>>) {
    let guard = m.lock().await;
    http_call(&guard).await;
    println!("Sent {:?} to the server", &guard);
    // Здесь удаляется `guard`
}
```

Теперь захват lock — asynchronous operation, которая передаёт управление runtime,
если не может продвинуться.\
В предыдущем сценарии произойдёт следующее:

```text
       Task A          Task B
          | 
  Acquires the lock
  Starts `http_call`
  Yields to runtime
          | 
          +--------------+
                         |
             Tries to acquire the lock
              Cannot acquire the lock
                 Yields to runtime
                         |
          +--------------+
          |
`http_call` completes      
  Releases the lock
   Yield to runtime
          |
          +--------------+
                         |
                 Acquires the lock
                       [...]
```

Теперь всё в порядке!

### Multithreaded не спасёт

В предыдущем примере execution context был single-threaded runtime, но тот же риск
сохраняется и при использовании multithreaded runtime.\
Различается лишь число concurrent tasks, необходимое для создания deadlock:
для single-threaded runtime достаточно 2, а для multithreaded runtime потребуется
`N+1` tasks, где `N` — число runtime threads.

### Downsides

Async-aware `Mutex` требует дополнительных затрат производительности.\
Если вы уверены, что для lock нет существенного contention, _и_ тщательно следите,
чтобы не удерживать его при прохождении через yield point, в asynchronous context
можно использовать `std::sync::Mutex`.

Но сопоставьте выигрыш в производительности с возникающим риском для liveness.

## Other primitives

Мы использовали `Mutex` как пример, но то же относится к `RwLock`, semaphores и т. д.\
При работе в asynchronous context предпочитайте async-aware версии, чтобы снизить риск проблем.
