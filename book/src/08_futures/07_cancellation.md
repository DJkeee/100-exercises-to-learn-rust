# Cancellation

Что происходит при drop pending future?\
Runtime больше не выполняет его poll, поэтому дальнейшее продвижение невозможно.
Иными словами, его выполнение **cancelled**.

На практике это часто происходит при работе с timeouts.
Например:

```rust
use tokio::time::timeout;
use tokio::sync::oneshot;
use std::time::Duration;

async fn http_call() {
    // [...]
}

async fn run() {
    // Оборачиваем future в `Timeout`, истекающий через 10 миллисекунд.
    let duration = Duration::from_millis(10);
    if let Err(_) = timeout(duration, http_call()).await {
        println!("Didn't receive a value within 10 ms");
    }
}
```

После истечения timeout future, возвращённый `http_call`, будет cancelled.
Представим, что тело `http_call` выглядит так:

```rust
use std::net::TcpStream;

async fn http_call() {
    let (stream, _) = TcpStream::connect(/* */).await.unwrap();
    let request: Vec<u8> = /* */;
    stream.write_all(&request).await.unwrap();
}
```

Каждый yield point становится **cancellation point**.\
Runtime не может preempt `http_call`, поэтому отбросить его можно только после передачи
управления executor через `.await`.
Это правило применяется рекурсивно: например, реализация `stream.write_all(&request)`,
вероятно, содержит несколько yield points. `http_call` вполне может отправить _часть_
request перед cancellation, затем закрыть соединение и не завершить передачу body.

## Clean up

Механизм cancellation в Rust весьма мощный: он позволяет вызывающему коду cancel
выполняющийся task без какого-либо содействия со стороны самого task.\
В то же время это может быть опасно. Иногда желательна **graceful cancellation**,
гарантирующая выполнение clean-up tasks перед прерыванием операции.

Например, рассмотрим вымышленный API для SQL transaction:

```rust
async fn transfer_money(
    connection: SqlConnection,
    payer_id: u64,
    payee_id: u64,
    amount: u64
) -> Result<(), anyhow::Error> {
    let transaction = connection.begin_transaction().await?;
    update_balance(payer_id, amount, &transaction).await?;
    decrease_balance(payee_id, amount, &transaction).await?;
    transaction.commit().await?;
}
```

При cancellation желательно явно прервать pending transaction, а не оставлять её незавершённой.
К сожалению, Rust не предоставляет полностью надёжного механизма для таких
**asynchronous** clean-up operations.

Наиболее распространённая стратегия — использовать trait `Drop` для scheduling
необходимой clean-up работы. Возможные варианты:

- Spawn нового task в runtime
- Добавление сообщения в channel queue
- Spawn background thread

Оптимальный выбор зависит от context.

## Cancelling spawned tasks

После spawn task с помощью `tokio::spawn` выполнить его drop уже нельзя:
он принадлежит runtime.\
Тем не менее при необходимости его можно cancel через `JoinHandle`:

```rust
async fn run() {
    let handle = tokio::spawn(/* some async task */);
    // Отменяем созданную задачу
    handle.abort();
}
```

## Further reading

- Будьте крайне осторожны при использовании macro `select!` из `tokio` для "race" двух futures.
  Повторное выполнение одного task в loop опасно, если нельзя гарантировать **cancellation safety**.
  Подробнее см. в [documentation `select!`](https://tokio.rs/tokio/tutorial/select).\
  Если требуется чередовать два asynchronous streams данных (например, socket и channel),
  лучше использовать [`StreamExt::merge`](https://docs.rs/tokio-stream/latest/tokio_stream/trait.StreamExt.html#method.merge).
- В некоторых случаях [`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html)
  предпочтительнее `JoinHandle::abort`.
