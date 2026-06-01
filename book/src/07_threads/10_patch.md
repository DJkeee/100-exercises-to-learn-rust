# Update operations

До сих пор мы реализовали только insert и get operations.\
Посмотрим, как расширить систему update operation.

## Updates в прежней версии

В non-threaded-версии системы updates были довольно простыми: `TicketStore` предоставлял
method `get_mut`, позволявший caller получить mutable reference на заявку, а затем изменить её.

## Multithreaded updates

В текущей multithreaded-версии эта стратегия не сработает. Borrow checker
остановит нас: `SyncSender<&mut Ticket>` не является `'static`, потому что `&mut Ticket` не удовлетворяет lifetime `'static`,
поэтому его нельзя capture в closure, передаваемой `std::thread::spawn`.

Есть несколько способов обойти это ограничение. Некоторые из них мы рассмотрим в следующих упражнениях.

### Patching

Нельзя отправить `&mut Ticket` через channel, поэтому mutation на стороне client невозможна.\
Можно ли выполнять mutation на стороне server?

Можно, если сообщить server, что именно требуется изменить. Иными словами, отправить server **patch**:

```rust
struct TicketPatch {
    id: TicketId,
    title: Option<TicketTitle>,
    description: Option<TicketDescription>,
    status: Option<TicketStatus>,
}
```

Field `id` обязателен, поскольку нужен для определения обновляемой заявки.\
Все остальные fields являются optional:

- Если field содержит `None`, его не следует изменять.
- Если field содержит `Some(value)`, его следует изменить на `value`.
