# Spawning tasks

Ваше решение предыдущего упражнения должно выглядеть примерно так:

```rust
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        let (mut reader, mut writer) = socket.split();
        tokio::io::copy(&mut reader, &mut writer).await?;
    }
}
```

Неплохо!\
Если между двумя входящими соединениями проходит много времени, function `echo` бездействует
(поскольку `TcpListener::accept` — asynchronous function), позволяя executor тем временем
выполнять другие tasks.

Но как действительно выполнять несколько tasks concurrently?\
Если всегда выполнять asynchronous functions до завершения (с помощью `.await`), одновременно
никогда не будет выполняться более одного task.

Здесь пригодится function `tokio::spawn`.

## `tokio::spawn`

`tokio::spawn` позволяет передать task executor, **не дожидаясь его завершения**.\
Вызывая `tokio::spawn`, вы указываете `tokio` продолжить выполнение spawned task
в фоне **concurrently** с породившим его task.

Вот как с его помощью обрабатывать несколько соединений concurrently:

```rust
use tokio::net::TcpListener;

pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut socket, _) = listener.accept().await?;
        // Создаем фоновую задачу для обработки соединения,
        // позволяя основной задаче сразу начать
        // принимать новые соединения
        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.split();
            tokio::io::copy(&mut reader, &mut writer).await?;
        });
    }
}
```

### Asynchronous blocks

В этом примере мы передали **asynchronous block** в `tokio::spawn`: `async move { /* */ }`.
Asynchronous blocks позволяют быстро пометить область кода как asynchronous, не определяя
отдельную async function.

### `JoinHandle`

`tokio::spawn` возвращает `JoinHandle`.\
С помощью `JoinHandle` можно применить `.await` к background task так же,
как мы использовали `join` для spawned threads.

```rust
pub async fn run() {
    // Создаем фоновую задачу для отправки телеметрии
    // на удаленный сервер
    let handle = tokio::spawn(emit_telemetry());
    // Тем временем выполняем другую полезную работу
    do_work().await;
    // Но не возвращаемся в вызывающий код, пока
    // телеметрия не будет успешно доставлена
    handle.await;
}

pub async fn emit_telemetry() {
    // [...]
}

pub async fn do_work() {
    // [...]
}
```

### Panic boundary

Если task, созданный с помощью `tokio::spawn`, вызывает panic, executor перехватит его.\
Если не применять `.await` к соответствующему `JoinHandle`, panic не будет передан spawner.
Даже при использовании `.await` для `JoinHandle` panic не передаётся автоматически.
Awaiting `JoinHandle` возвращает `Result` с error type
[`JoinError`](https://docs.rs/tokio/latest/tokio/task/struct.JoinError.html).
Затем можно проверить вызов panic через `JoinError::is_panic` и решить, что делать:
записать его в log, проигнорировать или передать дальше.

```rust
use tokio::task::JoinError;

pub async fn run() {
    let handle = tokio::spawn(work());
    if let Err(e) = handle.await {
        if let Ok(reason) = e.try_into_panic() {
            // В задаче возникла паника
            // Возобновляем раскрутку паники,
            // распространяя ее на текущую задачу
            panic::resume_unwind(reason);
        }
    }
}

pub async fn work() {
    // [...]
}
```

### `std::thread::spawn` vs `tokio::spawn`

`tokio::spawn` можно считать asynchronous-аналогом `std::thread::spawn`.

Обратите внимание на ключевое различие: при использовании `std::thread::spawn` управление
передаётся OS scheduler. Вы не контролируете scheduling threads.

При использовании `tokio::spawn` управление передаётся async executor, полностью работающему
в user space. Нижележащий OS scheduler не участвует в выборе следующего task для выполнения.
Теперь это решение принимаем мы через выбранный executor.
