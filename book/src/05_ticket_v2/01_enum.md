# Enumerations

Согласно validation logic, которую вы написали [в предыдущей главе](../03_ticket_v1/02_validation.md),
у ticket может быть лишь несколько допустимых statuses: `To-Do`, `InProgress` и `Done`.\
Это неочевидно, если посмотреть на field `status` в `Ticket` struct или на type parameter `status`
в method `new`:

```rust
#[derive(Debug, PartialEq)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: String
    ) -> Self {
        // [...]
    }
}
```

В обоих случаях для representation field `status` используется `String`.
`String` — очень общий type: по нему не сразу понятно, что field `status`
может содержать лишь ограниченный набор values. Хуже того, вызывающая сторона `Ticket::new` узнает,
допустим ли переданный status, только **at runtime**.

Исправить это можно с помощью **enumerations**.

## `enum`

Enumeration — это type с фиксированным набором values, которые называются **variants**.\
В Rust enumeration defined с помощью keyword `enum`:

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

`enum`, как и `struct`, объявляет **новый Rust type**.
