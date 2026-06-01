# `impl Trait`

`TicketStore::to_dos` возвращает `Vec<&Ticket>`.\
При каждом вызове `to_dos` эта signature приводит к новому allocation в heap, который может оказаться лишним в зависимости
от того, что caller будет делать с результатом.
Было бы лучше, если бы `to_dos` возвращал iterator вместо `Vec`. Тогда caller сможет самостоятельно решить,
собирать результаты в `Vec` или просто выполнить iteration по ним.

Однако здесь есть сложность!
Какой возвращаемый type у `to_dos` в приведённой ниже implementation?

```rust
impl TicketStore {
    pub fn to_dos(&self) -> ??? {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}
```

## Types без имени

Метод `filter` возвращает экземпляр `std::iter::Filter`, определённый следующим образом:

```rust
pub struct Filter<I, P> { /* fields omitted */ }
```

где `I` — тип фильтруемого iterator, а `P` — predicate, используемый для фильтрации элементов.\
Мы знаем, что в данном случае `I` — это `std::slice::Iter<'_, Ticket>`, но что представляет собой `P`?\
`P` — closure, **anonymous function**. Как следует из названия, у closures нет имён,
поэтому мы не можем записать их в коде.

В Rust для этого есть решение: **impl Trait**.

## `impl Trait`

`impl Trait` — feature, позволяющая возвращать type, не указывая его имя.
Достаточно объявить, какие trait или traits реализует type, а остальное определит Rust.

В данном случае требуется вернуть iterator references на `Ticket`:

```rust
impl TicketStore {
    pub fn to_dos(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.iter().filter(|t| t.status == Status::ToDo)
    }
}
```

Вот и всё!

## Generic?

`impl Trait` в return position — **не** generic parameter.

Generics — это placeholders для типов, которые заполняет caller функции.
Функция с generic parameter является **polymorphic**: её можно вызвать с разными типами, и compiler сгенерирует
отдельную implementation для каждого типа.

С `impl Trait` всё иначе.
Возвращаемый тип функции с `impl Trait` **фиксируется** во время компиляции, и compiler генерирует
для него одну implementation.
Поэтому `impl Trait` также называют **opaque return type**: caller не знает точный тип возвращаемого значения,
ему известно лишь то, что тип реализует указанные trait или traits. Но compiler знает точный тип: никакого polymorphism здесь нет.

## RPIT

В RFC и подробных материалах о Rust вам может встретиться аббревиатура **RPIT**.\
Она расшифровывается как **"Return Position Impl Trait"** и обозначает использование `impl Trait` в return position.
