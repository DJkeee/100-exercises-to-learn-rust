# Traits

Ещё раз взглянем на наш type `Ticket`:

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}
```

До сих пор во всех тестах assertions выполнялись с использованием fields `Ticket`.

```rust
assert_eq!(ticket.title(), "A new title");
```

Что, если мы захотим сравнить два instances `Ticket` напрямую?

```rust
let ticket1 = Ticket::new(/* ... */);
let ticket2 = Ticket::new(/* ... */);
ticket1 == ticket2
```

Compiler нам этого не позволит:

```text
error[E0369]: binary operation `==` cannot be applied to type `Ticket`
  --> src/main.rs:18:13
   |
18 |     ticket1 == ticket2
   |     ------- ^^ ------- Ticket
   |     |
   |     Ticket
   |
note: an implementation of `PartialEq` might be missing for `Ticket`
```

`Ticket` — новый type. Изначально с ним **не связано никакого behavior**.\
Rust не определяет магическим образом, как сравнивать два instances `Ticket`, только потому, что они содержат `String`.

Однако compiler Rust подсказывает правильное направление: возможно, нам не хватает implementation
`PartialEq`. `PartialEq` — это **trait**!

## Что такое traits?

Traits — это способ определения **interfaces** в Rust.\
Trait задаёт набор methods, которые type должен реализовать, чтобы выполнить contract trait.

### Определение trait

Синтаксис определения trait выглядит так:

```rust
trait <TraitName> {
    fn <method_name>(<parameters>) -> <return_type>;
}
```

Например, можно определить trait с именем `MaybeZero`, который требует, чтобы его implementors определили method `is_zero`:

```rust
trait MaybeZero {
    fn is_zero(self) -> bool;
}
```

### Implementation trait

Чтобы реализовать trait для type, мы используем keyword `impl`, как и для обычных[^inherent] methods,
но синтаксис немного отличается:

```rust
impl <TraitName> for <TypeName> {
    fn <method_name>(<parameters>) -> <return_type> {
        // Тело метода
    }
}
```

Например, реализуем trait `MaybeZero` для пользовательского числового type `WrappingU32`:

```rust
pub struct WrappingU32 {
    inner: u32,
}

impl MaybeZero for WrappingU32 {
    fn is_zero(self) -> bool {
        self.inner == 0
    }
}
```

### Вызов method trait

Для вызова method trait используется operator `.`, как и для обычных methods:

```rust
let x = WrappingU32 { inner: 5 };
assert!(!x.is_zero());
```

Чтобы вызвать method trait, должны выполняться два условия:

- Type должен реализовывать trait.
- Trait должен находиться в scope.

Для выполнения второго условия может потребоваться добавить statement `use` для trait:

```rust
use crate::MaybeZero;
```

Это не требуется, если:

- Trait определён в том же module, где происходит вызов.
- Trait определён в **prelude** standard library.
  Prelude — это набор traits и types, автоматически импортируемых в каждую программу Rust.
  Как будто в начало каждого module Rust добавили `use std::prelude::*;`.

Список traits и types из prelude можно найти в
[документации Rust](https://doc.rust-lang.org/std/prelude/index.html).

[^inherent]: Method, определённый непосредственно для type без использования trait, также называется **inherent method**.
