# `thiserror`

Мы немного отвлеклись, не так ли? Но это было необходимо!\
Вернёмся к основной теме: custom error types и `thiserror`.

## Custom error types

Мы уже увидели, как «вручную» implement trait `Error` для custom error type.\
Представьте, что это приходится делать для большинства error types в codebase. Получается много boilerplate, не так ли?

Часть boilerplate можно устранить с помощью [`thiserror`](https://docs.rs/thiserror/latest/thiserror/):
этот crate Rust предоставляет **procedural macro**, упрощающий создание custom error types.

```rust
#[derive(thiserror::Error, Debug)]
enum TicketNewError {
    #[error("{0}")]
    TitleError(String),
    #[error("{0}")]
    DescriptionError(String),
}
```

## Можно писать собственные macros

Все встречавшиеся до сих пор macros `derive` предоставлялись standard library Rust.\
`thiserror::Error` — первый пример **third-party** macro `derive`.

Macros `derive` являются подмножеством **procedural macros** — механизма генерации codeа Rust at compile-time.
В этом курсе мы не будем подробно разбирать написание procedural macro, но важно
знать, что вы можете создавать собственные macros!\
Эту тему стоит рассмотреть в более продвинутом курсе по Rust.

## Custom syntax

Каждый procedural macro может определять собственный syntax, который обычно описывается в documentation crate.
В случае `thiserror` используются:

- `#[derive(thiserror::Error)]` — syntax для derive trait `Error` у custom error type с помощью `thiserror`.
- `#[error("{0}")]` — syntax для объявления implementation `Display` у каждого variant custom error type.
  При выводе error вместо `{0}` подставляется нулевое field variant, в данном случае `String`.
