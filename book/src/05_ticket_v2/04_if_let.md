# Краткое branching

Ваше решение предыдущего упражнения, вероятно, выглядит так:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            Status::Done | Status::ToDo => {
                panic!(
                    "Only `In-Progress` tickets can be \
                    assigned to someone"
                )
            }
        }
    }
}
```

Вас интересует только variant `Status::InProgress`.
Действительно ли необходимо match все остальные variants?

На помощь приходят новые constructs!

## `if let`

Construct `if let` позволяет match единственный variant enum,
не обрабатывая все остальные variants.

Вот как с помощью `if let` можно упростить method `assigned_to`:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        if let Status::InProgress { assigned_to } = &self.status {
            assigned_to
        } else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        }
    }
}
```

## `let/else`

Если branch `else` предназначен для early return (`panic` тоже считается early return!),
можно использовать construct `let/else`:

```rust
impl Ticket {
    pub fn assigned_to(&self) -> &str {
        let Status::InProgress { assigned_to } = &self.status else {
            panic!(
                "Only `In-Progress` tickets can be assigned to someone"
            );
        };
        assigned_to
    }
}
```

Он позволяет присвоить destructured variable без какого-либо «смещения вправо»:
variable присваивается на том же уровне indentation,
что и предшествующий ей code.

## Style

И `if let`, и `let/else` — idiomatic constructs Rust.\
Используйте их там, где считаете уместным для улучшения читаемости codeа,
но не злоупотребляйте: при необходимости всегда можно использовать `match`.
