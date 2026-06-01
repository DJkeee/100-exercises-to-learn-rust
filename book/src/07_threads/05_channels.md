# Channels

До сих пор все наши spawned threads были довольно недолговечными.\
Получить input data, выполнить computation, вернуть result, завершиться.

Для нашей системы управления заявками нужен другой подход:
client-server architecture.

У нас будет **один long-running server thread**, отвечающий за управление
нашим state — сохранёнными заявками.

Также у нас будет **несколько client threads**.\
Каждый client сможет отправлять **commands** и **queries** в stateful thread,
чтобы изменять его state (например, добавлять новую заявку) или получать
информацию (например, status заявки).\
Client threads будут выполняться concurrently.

## Communication

До сих пор parent-child communication была сильно ограничена:

- Spawned thread borrowed или потреблял data из parent context
- Spawned thread возвращал data parent thread при join

Для client-server design этого недостаточно.\
Clients должны иметь возможность отправлять data server thread и получать их от него
_после_ его запуска.

Проблему можно решить с помощью **channels**.

## Channels

Standard library Rust предоставляет **multi-producer, single-consumer** (mpsc) channels
в module `std::sync::mpsc`.\
Есть два варианта channels: bounded и unbounded. Пока используем unbounded-вариант,
а его достоинства и недостатки обсудим позже.

Создание channel выглядит так:

```rust
use std::sync::mpsc::channel;

let (sender, receiver) = channel();
```

Вы получаете sender и receiver.\
Чтобы поместить data в channel, нужно вызвать `send` у sender.\
Чтобы извлечь data из channel, нужно вызвать `recv` у receiver.

### Multiple senders

`Sender` является clonable: можно создать несколько senders (например, по одному
для каждого client thread), и все они будут помещать data в один channel.

А `Receiver` не является clonable: у конкретного channel может быть только один receiver.

Именно это означает **mpsc** (multi-producer single-consumer)!

### Message type

И `Sender`, и `Receiver` являются generic по type parameter `T`.\
Это type _messages_, передаваемых через наш channel.

Им может быть `u64`, struct, enum и т. д.

### Errors

И `send`, и `recv` могут завершиться с error.\
`send` возвращает error, если receiver был dropped.\
`recv` возвращает error, если все senders были dropped, а channel пуст.

Иными словами, `send` и `recv` возвращают error, когда channel фактически закрыт.
