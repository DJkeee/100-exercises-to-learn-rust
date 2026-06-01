# Implementation traits

Если type определён в другом crate (например, `u32` из standard library Rust), для него
нельзя напрямую определять новые methods. Если попытаться:

```rust
impl u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}
```

compiler сообщит об ошибке:

```text
error[E0390]: cannot define inherent `impl` for primitive types
  |
1 | impl u32 {
  | ^^^^^^^^
  |
  = help: consider using an extension trait instead
```

## Extension trait

**Extension trait** — это trait, основная задача которого заключается в добавлении новых methods
к внешним types, таким как `u32`.
Именно этот pattern вы использовали в предыдущем упражнении: определили
trait `IsEven`, а затем реализовали его для `i32` и `u32`. После этого
можно свободно вызывать `is_even` для этих types, пока `IsEven` находится в scope.

```rust
// Вводим трейт в область видимости
use my_library::IsEven;

fn main() {
    // Вызываем его метод для типа, который его реализует
    if 4.is_even() {
        // [...]
    }
}
```

## Одна implementation

На implementations trait накладываются ограничения.\
Самое простое и очевидное: нельзя дважды реализовать один и тот же trait
для одного type в одном crate.

Например:

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        true
    }
}

impl IsEven for u32 {
    fn is_even(&self) -> bool {
        false
    }
}
```

Compiler отклонит этот код:

```text
error[E0119]: conflicting implementations of trait `IsEven` for type `u32`
   |
5  | impl IsEven for u32 {
   | ------------------- first implementation here
...
11 | impl IsEven for u32 {
   | ^^^^^^^^^^^^^^^^^^^ conflicting implementation for `u32`
```

При вызове `IsEven::is_even` для value `u32` не должно возникать неоднозначности в выборе implementation trait,
поэтому implementation может быть только одна.

## Orphan rule

Когда задействовано несколько crates, ситуация становится сложнее.
В частности, должно выполняться хотя бы одно из следующих условий:

- Trait определён в текущем crate
- Type implementor определён в текущем crate

Это правило называется **orphan rule** Rust. Его цель — исключить неоднозначность
в процессе method resolution.

Представьте следующую ситуацию:

- Crate `A` определяет trait `IsEven`
- Crate `B` реализует `IsEven` для `u32`
- Crate `C` предоставляет другую implementation trait `IsEven` для `u32`
- Crate `D` зависит от `B` и `C` и вызывает `1.is_even()`

Какую implementation следует использовать? Определённую в `B`? Или определённую в `C`?\
Хорошего ответа нет, поэтому для предотвращения такого сценария и было введено orphan rule.
Благодаря orphan rule ни crate `B`, ни crate `C` не смогут скомпилироваться.

## Дополнительные материалы

- У описанного выше orphan rule есть некоторые оговорки и исключения.
  Обратитесь к [reference](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementation-coherence),
  если хотите разобраться в нюансах.
