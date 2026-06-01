# Asynchronous functions

Все functions и methods, которые вы писали до сих пор, были eager.\
До их вызова ничего не происходило. Но после вызова они выполнялись до
завершения: делали **всю** свою работу и затем возвращали результат.

Иногда это нежелательно.\
Например, при написании HTTP server может потребоваться много
**ожидания**: получения request body, ответа database, ответа downstream service и т. д.

Что, если во время ожидания можно было бы заняться чем-то ещё?\
Что, если можно было бы прервать вычисление на полпути?\
Что, если можно было бы отдать другой task приоритет перед текущим?

Для этого и нужны **asynchronous functions**.

## `async fn`

Для определения asynchronous function используется keyword `async`:

```rust
use tokio::net::TcpListener;

// Эта функция асинхронная
async fn bind_random() -> TcpListener {
    // [...]
}
```

Что произойдёт, если вызвать `bind_random` как обычную function?

```rust
fn run() {
    // Вызываем `bind_random`
    let listener = bind_random();
    // Что теперь?
}
```

Ничего!\
Rust не начинает выполнять `bind_random` при вызове даже в виде background task
(как можно было бы ожидать, опираясь на опыт работы с другими языками).
Asynchronous functions в Rust **lazy**: они не выполняют никакой работы, пока вы
явно их об этом не попросите.
В терминологии Rust function `bind_random` возвращает **future** — type,
представляющий вычисление, которое может завершиться позднее. Такие types называются futures,
поскольку реализуют trait `Future` — interface, который мы подробно рассмотрим далее в этой главе.

## `.await`

Самый распространённый способ попросить asynchronous function выполнить некоторую работу —
использовать keyword `.await`:

```rust
use tokio::net::TcpListener;

async fn bind_random() -> TcpListener {
    // [...]
}

async fn run() {
    // Вызываем `bind_random` и ждем завершения
    let listener = bind_random().await;
    // Теперь `listener` готов
}
```

`.await` не возвращает управление вызывающему коду, пока asynchronous function
не завершится — например, пока в приведённом выше примере не будет создан `TcpListener`.

## Runtimes

Если это вызывает недоумение, то не зря!\
Только что мы сказали, что преимущество asynchronous functions
заключается в том, что они не выполняют **всю** работу сразу. Затем мы познакомились
с `.await`, который не возвращает управление, пока asynchronous function не завершится.
Разве мы не вернулись к проблеме, которую пытались решить? В чём смысл?

Не совсем! При вызове `.await` за кулисами происходит многое!\
Вы передаёте управление **async runtime**, также известному как **async executor**.
Именно executors управляют всеми выполняющимися asynchronous **tasks**.
В частности, они обеспечивают баланс между двумя целями:

- **Progress**: обеспечивают продвижение tasks при любой возможности.
- **Efficiency**: если task чего-то ожидает, стараются тем временем запустить
  другой task, полностью задействуя доступные ресурсы.

### No default runtime

Подход Rust к asynchronous programming довольно необычен: default runtime
отсутствует. Standard library не предоставляет его. Вам придётся выбрать его самостоятельно!

В большинстве случаев вы выберете один из вариантов, доступных в ecosystem.
Некоторые runtimes рассчитаны на широкий спектр задач и подходят большинству приложений.
К этой категории относятся `tokio` и `async-std`. Другие runtimes оптимизированы
для конкретных сценариев — например, `embassy` для embedded systems.

На протяжении курса мы будем использовать `tokio` — самый популярный runtime
общего назначения для asynchronous programming в Rust.

### `#[tokio::main]`

Entrypoint исполняемого файла, function `main`, должна быть synchronous function.
Именно в ней следует настроить и запустить выбранный async runtime.

Большинство runtimes предоставляют macro, упрощающий эту задачу. Для `tokio` это `tokio::main`:

```rust
#[tokio::main]
async fn main() {
    // Здесь размещается ваш асинхронный код
}
```

который разворачивается в:

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(
        // Здесь размещается ваша асинхронная функция
        // [...]
    );
}
```

### `#[tokio::test]`

То же относится к tests: они должны быть synchronous functions.\
Каждая test function запускается в отдельном thread, и если в tests требуется
выполнять async code, вы отвечаете за настройку и запуск async runtime.\
`tokio` предоставляет macro `#[tokio::test]`, упрощающий эту задачу:

```rust
#[tokio::test]
async fn my_test() {
    // Здесь размещается ваш код асинхронного теста
}
```
