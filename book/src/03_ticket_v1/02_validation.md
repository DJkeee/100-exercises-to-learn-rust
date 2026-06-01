# Validation

Вернёмся к определению нашего ticket:

```rust
struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

Для fields struct `Ticket` мы используем «сырые» types.
Это значит, что пользователи могут создать ticket с пустым title, оооооооочень длинным description или
бессмысленным status (например, "Funny").\
Можно сделать лучше!

## Дополнительные материалы

- Изучите [документацию `String`](https://doc.rust-lang.org/std/string/struct.String.html):
  там подробно описаны предоставляемые им methods. Она понадобится вам для выполнения упражнения!
