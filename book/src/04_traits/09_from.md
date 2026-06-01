# `From` and `Into`

Вернёмся к началу нашего путешествия по строкам:

```rust
let ticket = Ticket::new(
    "A title".into(), 
    "A description".into(), 
    "To-Do".into()
);
```

Теперь наших знаний достаточно, чтобы разобраться, что здесь делает `.into()`.

## Проблема

Вот signature method `new`:

```rust
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

Мы также выяснили, что string literals (такие как `"A title"`) имеют type `&str`.\
Здесь возникает type mismatch: ожидается `String`, а у нас есть `&str`.
На этот раз никакой волшебной coercion не будет: нужно **выполнить conversion**.

## `From` и `Into`

Standard library Rust определяет два traits для **infallible conversions**: `From` и `Into`,
в module `std::convert`.

```rust
pub trait From<T>: Sized {
    fn from(value: T) -> Self;
}

pub trait Into<T>: Sized {
    fn into(self) -> T;
}
```

В этих определениях traits встречается несколько новых для нас концепций: **supertraits** и **implicit trait bounds**.
Сначала разберём их.

### Supertrait / Subtrait

Синтаксис `From: Sized` означает, что `From` — **subtrait** для `Sized`: любой type,
реализующий `From`, должен также реализовывать `Sized`.
Иначе говоря, `Sized` — **supertrait** для `From`.

### Implicit trait bounds

При наличии generic type parameter compiler неявно предполагает, что он является `Sized`.

Например:

```rust
pub struct Foo<T> {
    inner: T,
}
```

на самом деле эквивалентно:

```rust
pub struct Foo<T: Sized> 
{
    inner: T,
}
```

В случае `From<T>` определение trait эквивалентно:

```rust
pub trait From<T: Sized>: Sized {
    fn from(value: T) -> Self;
}
```

Иными словами, _и_ `T`, _и_ type, реализующий `From<T>`, должны быть `Sized`, хотя
первый bound задан неявно.

### Negative trait bounds

От неявного bound `Sized` можно отказаться с помощью **negative trait bound**:

```rust
pub struct Foo<T: ?Sized> {
    //            ^^^^^^^
    //            Это отрицательное ограничение трейта
    inner: T,
}
```

Этот синтаксис читается как «`T` может быть `Sized`, а может и не быть» и позволяет
связать `T` с DST (например, `Foo<str>`). Однако это особый случай: negative trait bounds применимы только к `Sized`,
с другими traits их использовать нельзя.

## Из `&str` в `String`

В [documentation `std`](https://doc.rust-lang.org/std/convert/trait.From.html#implementors)
можно посмотреть, какие types `std` реализуют trait `From`.\
Там указано, что `String` реализует `From<&str> for String`. Следовательно, можно написать:

```rust
let title = String::from("A title");
```

Однако до сих пор мы в основном использовали `.into()`.\
Если посмотреть [implementors `Into`](https://doc.rust-lang.org/std/convert/trait.Into.html#implementors),
то `Into<String> for &str` там не найдётся. Что происходит?

`From` и `Into` — **dual traits**.\
В частности, `Into` реализуется для любого type, реализующего `From`, с помощью **blanket implementation**:

```rust
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    fn into(self) -> U {
        U::from(self)
    }
}
```

Если type `U` реализует `From<T>`, то `Into<U> for T` реализуется автоматически. Именно поэтому
можно написать `let title = "A title".into();`.

## `.into()`

Всякий раз, когда встречается `.into()`, происходит conversion между types.\
Но каков target type?

В большинстве случаев target type:

- Задан signature function или method (например, `Ticket::new` в примере выше)
- Задан в объявлении variable с помощью type annotation (например, `let title: String = "A title".into();`)

`.into()` будет работать без дополнительной настройки, пока compiler может однозначно вывести target type из контекста.
