# `TryFrom` и `TryInto`

В предыдущей главе мы рассмотрели [traits `From` и `Into`](../04_traits/09_from.md) —
idiomatic interfaces Rust для **infallible** conversions types.\
Но что делать, если успех conversion не гарантирован?

Теперь мы знаем достаточно об errors, чтобы обсудить **fallible** аналоги `From` и `Into`:
`TryFrom` и `TryInto`.

## `TryFrom` and `TryInto`

И `TryFrom`, и `TryInto`, как и `From` и `Into`, defined в module `std::convert`.

```rust
pub trait TryFrom<T>: Sized {
    type Error;
    fn try_from(value: T) -> Result<Self, Self::Error>;
}

pub trait TryInto<T>: Sized {
    type Error;
    fn try_into(self) -> Result<T, Self::Error>;
}
```

Главное отличие `From`/`Into` от `TryFrom`/`TryInto` состоит в том, что последние возвращают type `Result`.\
Благодаря этому conversion может завершиться неудачей, вернув error вместо вызова `panic`.

## `Self::Error`

У `TryFrom` и `TryInto` есть associated type `Error`.
Это позволяет каждой implementation указывать собственный error type, в идеале наиболее подходящий
для выполняемого conversion.

`Self::Error` — способ обратиться к associated type `Error`, defined в самом trait.

## Duality

Как и `From` с `Into`, `TryFrom` и `TryInto` являются dual traits.\
Если implement `TryFrom` для type, `TryInto` будет доступен автоматически.
