# Bounded и unbounded channels

До сих пор мы использовали unbounded channels.\
В них можно отправлять сколько угодно messages, а channel будет увеличиваться по мере необходимости.\
В сценарии multi-producer single-consumer это может стать проблемой: если producers
помещают messages в queue быстрее, чем consumer успевает их обрабатывать, channel продолжит
расти и может занять всю доступную memory.

Мы рекомендуем **никогда** не использовать unbounded channel в production-системе.\
Всегда следует ограничивать максимальное количество messages в queue с помощью
**bounded channel**.

## Bounded channels

У bounded channel фиксированная capacity.\
Его можно создать вызовом `sync_channel` с capacity больше нуля:

```rust
use std::sync::mpsc::sync_channel;

let (sender, receiver) = sync_channel(10);
```

Type `receiver` остался прежним: `Receiver<T>`.\
А `sender` является instance `SyncSender<T>`.

### Sending messages

Для отправки messages через `SyncSender` есть два methods:

- `send`: если в channel есть место, method поместит message в queue и вернёт `Ok(())`.\
  Если channel заполнен, method заблокируется и будет ждать появления свободного места.
- `try_send`: если в channel есть место, method поместит message в queue и вернёт `Ok(())`.\
  Если channel заполнен, method вернёт `Err(TrySendError::Full(value))`, где `value` — message, которое не удалось отправить.

Выбор method зависит от вашего use case.

### Backpressure

Главное преимущество bounded channels в том, что они обеспечивают форму **backpressure**.\
Они заставляют producers замедлиться, если consumer не успевает обрабатывать data.
Затем backpressure может распространиться по системе, затронув всю architecture
и не позволив конечным пользователям перегрузить систему запросами.
