# Nullability

Наша implementation method `assigned` довольно грубая: `panic` для tickets со status to-do и done далёк от идеала.\
Можно сделать лучше, используя **type `Option` из Rust**.

## `Option`

`Option` — это Rust type, представляющий **nullable values**.\
Это enum, defined в standard library Rust:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option` выражает идею о том, что value может присутствовать (`Some(T)`) или отсутствовать (`None`).\
Кроме того, он заставляет **явно обработать оба случая**. Если при работе с nullable value забыть
обработать случай `None`, compiler выдаст error.\
Это значительное улучшение по сравнению с «неявной» nullability в других языках, где можно забыть check
`null` и тем самым вызвать runtime error.

## Объявление `Option`

В definition `Option` используется construct Rust, с которым вы ещё не сталкивались: **tuple-like variants**.

### Tuple-like variants

У `Option` есть два variants: `Some(T)` и `None`.\
`Some` — это **tuple-like variant**, то есть variant с **безымянными fields**.

Tuple-like variants часто используются, когда необходимо хранить единственное field, особенно если речь идёт
о typeе-«обёртке» наподобие `Option`.

### Tuple-like structs

Они характерны не только для enums: можно define и tuple-like structs:

```rust
struct Point(i32, i32);
```

После этого к двум fields instance `Point` можно обращаться по их positional index:

```rust
let point = Point(3, 4);
let x = point.0;
let y = point.1;
```

### Tuples

Странно говорить, что нечто является tuple-like, если мы ещё не знакомы с tuples!\
Tuples — ещё один пример primitive type в Rust.
Они объединяют фиксированное количество values с types, которые могут различаться:

```rust
// Два значения одного типа
let first: (i32, i32) = (3, 4);
// Три значения разных типов
let second: (i32, u32, u8) = (-42, 3, 8);
```

Syntax проста: types values перечисляются через запятую в круглых скобках.
К fields tuple можно обращаться с помощью dot notation и index field:

```rust
assert_eq!(second.0, -42);
assert_eq!(second.1, 3);
assert_eq!(second.2, 8);
```

Tuples позволяют удобно группировать value, когда define отдельный type struct нецелесообразно.
