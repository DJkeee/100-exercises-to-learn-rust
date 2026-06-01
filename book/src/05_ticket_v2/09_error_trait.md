# Trait Error

## Error reporting

В предыдущем упражнении потребовалось выполнить destructuring variant `TitleError`, чтобы извлечь message об error
и передать его macro `panic!`.\
Это простой пример **error reporting**: conversion error type в representation, которое можно
показать пользователю, operatorу сервиса или developer.

Каждому Rust developer непрактично придумывать собственную стратегию error reporting: это отнимает время,
а объединять такие стратегии в разных проектах неудобно.
Поэтому Rust предоставляет trait `std::error::Error`.

## Trait `Error`

Тип variant `Err` в `Result` ничем не ограничен, но хорошей практикой считается использование type,
который implements trait `Error`.
`Error` лежит в основе error handling в Rust:

```rust
// Слегка упрощенное определение трейта `Error`
pub trait Error: Debug + Display {}
```

Возможно, вы помните syntax `:` по [trait `From`](../04_traits/09_from.md#supertrait--subtrait): он используется для указания **supertraits**.
У `Error` два supertraits: `Debug` и `Display`. Чтобы type мог implement `Error`, он также должен
implement `Debug` и `Display`.

## `Display` and `Debug`

Мы уже сталкивались с trait `Debug` [в предыдущем упражнении](../04_traits/04_derive.md): `assert_eq!` использует его,
чтобы при неудачной assertion вывести value сравниваемых variables.

С «механической» точки зрения `Display` и `Debug` одинаковы: они определяют, как type следует convert
в string-like representation:

```rust
// `Debug`
pub trait Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}

// `Display`
pub trait Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>;
}
```

Разница состоит в их _назначении_: `Display` возвращает representation для «конечных пользователей»,
а `Debug` предоставляет low-level representation, которое больше подходит developers и operatorам сервисов.\
Поэтому `Debug` можно implement автоматически с помощью attribute `#[derive(Debug)]`, а `Display`
**требует** manual implementation.
