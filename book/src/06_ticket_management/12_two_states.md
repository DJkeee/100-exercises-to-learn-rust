# Ticket ids

Вернёмся к нашей системе управления тикетами.\
Сейчас модель тикета выглядит так:

```rust
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

Здесь не хватает **identifier**, однозначно определяющего тикет.\
Identifier должен быть уникальным для каждого тикета. Это можно гарантировать, автоматически генерируя его
при создании нового тикета.

## Уточнение модели

Где следует хранить id?\
Можно добавить новое поле в struct `Ticket`:

```rust
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

Но до создания тикета id неизвестен. Поэтому он не может присутствовать с самого начала.\
Он должен быть optional:

```rust
pub struct Ticket {
    pub id: Option<TicketId>,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

Это тоже неидеально: при каждом получении тикета из store пришлось бы обрабатывать случай `None`,
хотя мы знаем, что после создания тикета id всегда должен присутствовать.

Лучшее решение — использовать два разных **states** тикета, представленных отдельными types:
`TicketDraft` и `Ticket`:

```rust
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription
}

pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status
}
```

`TicketDraft` — ещё не созданный тикет. У него нет ни id, ни status.\
`Ticket` — созданный тикет. У него есть id и status.\
Поскольку каждое field в `TicketDraft` и `Ticket` содержит собственные constraints, дублировать логику
в двух types не требуется.
