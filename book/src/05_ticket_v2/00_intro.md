# Моделирование Ticket, ч. 2

`Ticket` struct, с которой мы работали в предыдущих главах, стала хорошим началом,
но она всё ещё буквально кричит: «Я начинающий Rustacean!».

В этой главе мы улучшим навыки domain modelling в Rust.
По ходу дела нам потребуется познакомиться ещё с несколькими concepts:

- `enum` — одна из самых мощных возможностей Rust для data modelling
- Тип `Option` для моделирования nullable values
- Type `Result` для моделирования recoverable errors
- Traits `Debug` и `Display` для вывода
- Trait `Error` для обоvalue error types
- Traits `TryFrom` и `TryInto` для fallible conversions
- Package system Rust: что такое library, что такое binary и как использовать third-party crates
