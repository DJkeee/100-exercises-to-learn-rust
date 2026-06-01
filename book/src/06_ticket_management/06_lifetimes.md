# Lifetimes

Попробуем завершить предыдущее упражнение, добавив implementation `IntoIterator` для `&TicketStore`, чтобы
сделать циклы `for` максимально удобными.

Начнём с заполнения наиболее «очевидных» частей implementation:

```rust
impl IntoIterator for &TicketStore {
    type Item = &Ticket;
    type IntoIter = // Что должно быть здесь?

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}
```

Какое значение следует присвоить `type IntoIter`?\
Интуитивно это должен быть тип, возвращаемый `self.tickets.iter()`, то есть тип, который возвращает `Vec::iter()`.\
Из документации standard library можно узнать, что `Vec::iter()` возвращает `std::slice::Iter`.
Определение `Iter` выглядит так:

```rust
pub struct Iter<'a, T> { /* fields omitted */ }
```

`'a` — это **lifetime parameter**.

## Lifetime parameters

Lifetimes — это **labels**, с помощью которых Rust compiler отслеживает, как долго reference, mutable или
immutable, остаётся валидной.\
Lifetime reference ограничен scope значения, на которое она указывает. Во время компиляции Rust всегда проверяет,
что references не используются после drop значения, на которое они указывают, предотвращая dangling pointers и ошибки use-after-free.

Это должно быть знакомо: мы уже видели эти концепции в действии, когда обсуждали ownership и borrowing.
Lifetimes — всего лишь способ **дать имя** периоду валидности конкретной reference.

Имена приобретают значение, когда references несколько и нужно уточнить, как они **связаны друг с другом**.
Рассмотрим signature `Vec::iter()`:

```rust
impl <T> Vec<T> {
    // Слегка упрощено
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        // [...]
    }
}
```

`Vec::iter()` является generic по lifetime parameter с именем `'a`.\
`'a` **связывает** lifetime `Vec` с lifetime `Iter`, возвращаемого методом `iter()`.
Проще говоря, `Iter`, возвращённый методом `iter()`, не может пережить reference на `Vec` (`&self`), из которой он был создан.

Это важно, поскольку, как мы уже обсуждали, `Vec::iter` возвращает iterator по **references** на элементы `Vec`.
После drop `Vec` references, возвращённые iterator, станут невалидными. Rust должен предотвратить такую ситуацию,
и для этого используются lifetimes.

## Lifetime elision

В Rust существует набор правил, называемых **lifetime elision rules**. Во многих случаях они позволяют опустить явные lifetime annotations.
Например, в исходном коде `std` определение `Vec::iter` выглядит так:

```rust
impl <T> Vec<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        // [...]
    }
}
```

В signature `Vec::iter()` нет явного lifetime parameter.
Elision rules подразумевают, что lifetime `Iter`, возвращённого методом `iter()`, связан с lifetime reference `&self`.
`'_` можно воспринимать как **placeholder** для lifetime reference `&self`.

Ссылка на официальную документацию по lifetime elision приведена в разделе [References](#references).\
В большинстве случаев можно положиться на compiler: он сообщит, когда нужно добавить явные lifetime annotations.

## References

- [std::vec::Vec::iter](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter)
- [std::slice::Iter](https://doc.rust-lang.org/std/slice/struct.Iter.html)
- [Lifetime elision rules](https://doc.rust-lang.org/reference/lifetime-elision.html)
