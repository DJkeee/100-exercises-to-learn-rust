# Operator overloading

Теперь, когда у нас есть базовое понимание traits, вернёмся к **operator overloading**.
Operator overloading — это возможность определять пользовательское behavior для operators вроде `+`, `-`, `*`, `/`, `==`, `!=` и т. д.

## Operators — это traits

В Rust operators являются traits.\
Каждому operator соответствует trait, определяющий его behavior.
Реализовав этот trait для своего type, вы **открываете** возможность использовать соответствующие operators.

Например, [trait `PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) определяет behavior
operators `==` и `!=`:

```rust
// Определение трейта `PartialEq` из стандартной библиотеки Rust
// (пока оно *слегка* упрощено)
pub trait PartialEq {
    // Обязательный метод
    //
    // `Self` — ключевое слово Rust, означающее
    // «тип, реализующий трейт»
    fn eq(&self, other: &Self) -> bool;

    // Предоставляемый метод
    fn ne(&self, other: &Self) -> bool { ... }
}
```

Когда вы пишете `x == y`, compiler ищет implementation trait `PartialEq` для types `x` и `y`
и заменяет `x == y` на `x.eq(y)`. Это syntactic sugar!

Основным operators соответствуют следующие traits:

| Operator                 | Trait                                                                   |
| ------------------------ | ----------------------------------------------------------------------- |
| `+`                      | [`Add`](https://doc.rust-lang.org/std/ops/trait.Add.html)               |
| `-`                      | [`Sub`](https://doc.rust-lang.org/std/ops/trait.Sub.html)               |
| `*`                      | [`Mul`](https://doc.rust-lang.org/std/ops/trait.Mul.html)               |
| `/`                      | [`Div`](https://doc.rust-lang.org/std/ops/trait.Div.html)               |
| `%`                      | [`Rem`](https://doc.rust-lang.org/std/ops/trait.Rem.html)               |
| `==` and `!=`            | [`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)   |
| `<`, `>`, `<=`, and `>=` | [`PartialOrd`](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) |

Arithmetic operators находятся в module [`std::ops`](https://doc.rust-lang.org/std/ops/index.html),
а comparison operators — в module [`std::cmp`](https://doc.rust-lang.org/std/cmp/index.html).

## Default implementations

В comment к `PartialEq::ne` сказано, что «`ne` is a provided method».\
Это означает, что `PartialEq` предоставляет **default implementation** для `ne` в определении trait — пропущенный block
`{ ... }` во фрагменте определения.\
Если раскрыть пропущенный block, получится следующее:

```rust
pub trait PartialEq {
    fn eq(&self, other: &Self) -> bool;

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
```

Именно этого и следовало ожидать: `ne` является отрицанием `eq`.\
Поскольку предоставлена default implementation, при создании implementation `PartialEq` для своего type можно не реализовывать `ne`.
Достаточно реализовать `eq`:

```rust
struct WrappingU8 {
    inner: u8,
}

impl PartialEq for WrappingU8 {
    fn eq(&self, other: &WrappingU8) -> bool {
        self.inner == other.inner
    }
    
    // Здесь нет реализации `ne`
}
```

Однако использовать default implementation необязательно.
При создании implementation trait её можно override:

```rust
struct MyType;

impl PartialEq for MyType {
    fn eq(&self, other: &MyType) -> bool {
        // Пользовательская реализация
    }

    fn ne(&self, other: &MyType) -> bool {
        // Пользовательская реализация
    }
}
```
