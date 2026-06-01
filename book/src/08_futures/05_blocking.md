# Don't block the runtime

Вернёмся к yield points.\
В отличие от threads, **tasks Rust нельзя preempt**.

`tokio` не может самостоятельно приостановить task и запустить вместо него другой.
Управление возвращается executor **исключительно** при yielding task, то есть когда
`Future::poll` возвращает `Poll::Pending` или, в случае `async fn`, когда к future
применяется `.await`.

Это создаёт риск для runtime: если task никогда не выполняет yield, runtime не сможет
запустить другой task. Это называется **blocking runtime**.

## What is blocking?

Какой промежуток времени уже слишком велик? Сколько времени task может работать
без yielding, прежде чем это станет проблемой?

Это зависит от runtime, приложения, числа in-flight tasks и многих других факторов.
Но в качестве общего правила старайтесь тратить между yield points менее 100 microseconds.

## Consequences

Blocking runtime может привести к:

- **Deadlocks**: если task без yielding ожидает завершения другого task, а тот ждёт,
  пока первый выполнит yield, возникает deadlock. Продвижение невозможно, если только
  runtime не сможет назначить другой task на другой thread.
- **Starvation**: другие tasks могут не получить возможность выполниться или запуститься
  с большой задержкой, что снижает производительность (например, повышает tail latencies).

## Blocking is not always obvious

Некоторых видов операций в async code обычно следует избегать:

- Synchronous I/O. Невозможно предсказать его длительность, и она, скорее всего,
  превысит 100 microseconds.
- Затратные CPU-bound вычисления.

Вторая категория не всегда очевидна. Например, сортировка vector из нескольких элементов
не создаёт проблем; но оценка изменится, если в vector миллиарды элементов.

## How to avoid blocking

Как избежать blocking runtime, если _необходимо_ выполнить операцию, которая является
или может оказаться blocking?\
Нужно перенести работу в другой thread. Не следует использовать runtime threads,
на которых `tokio` выполняет tasks.

Для этого `tokio` предоставляет отдельный threadpool — **blocking pool**.
Запустить synchronous operation в blocking pool можно с помощью function
`tokio::task::spawn_blocking`. `spawn_blocking` возвращает future, который после завершения
операции resolves в её результат.

```rust
use tokio::task;

fn expensive_computation() -> u64 {
    // [...]
}

async fn run() {
    let handle = task::spawn_blocking(expensive_computation);
    // Тем временем выполняем другие действия
    let result = handle.await.unwrap();
}
```

Blocking pool существует длительное время. `spawn_blocking` должен работать быстрее,
чем непосредственное создание нового thread через `std::thread::spawn`, поскольку
стоимость инициализации thread амортизируется между несколькими вызовами.

## Further reading

- Подробнее об этом рассказано в [blog post Alice Ryhl](https://ryhl.io/blog/async-what-is-blocking/).
