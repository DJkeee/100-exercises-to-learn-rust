# `Sized`

Даже после изучения deref coercion type `&str` оказывается сложнее, чем кажется.\
После нашего предыдущего [обсуждения memory layouts](../03_ticket_v1/10_references_in_memory.md)
было бы логично ожидать, что `&str` представлен в stack одним `usize` —
pointer. Однако это не так. Рядом с pointer type `&str` хранит **metadata**:
длину slice, на который указывает. Вернёмся к примеру из
[предыдущего раздела](06_str_slice.md):

```rust
let mut s = String::with_capacity(5);
s.push_str("Hello");
// Создаем ссылку на строковый срез из `String`,
// пропуская первый байт.
let slice: &str = &s[1..];
```

В memory получаем:

```text
                    s                              slice
      +---------+--------+----------+      +---------+--------+
Stack | pointer | length | capacity |      | pointer | length |
      |    |    |   5    |    5     |      |    |    |   4    |
      +----|----+--------+----------+      +----|----+--------+
           |        s                           |  
           |                                    |
           v                                    | 
         +---+---+---+---+---+                  |
Heap:    | H | e | l | l | o |                  |
         +---+---+---+---+---+                  |
               ^                                |
               |                                |
               +--------------------------------+
```

Что происходит?

## Dynamically sized types

`str` — это **dynamically sized type** (DST).\
DST — type, размер которого неизвестен во время compilation. Любая
reference на DST, например `&str`, должна содержать дополнительную
информацию о данных, на которые указывает. Это **fat pointer**.\
В случае `&str` хранится длина slice, на который он указывает.
В следующих частях курса встретятся и другие примеры DST.

## Trait `Sized`

Library `std` в Rust определяет trait `Sized`.

```rust
pub trait Sized {
    // Это пустой трейт, реализовывать методы не требуется.
}
```

Type является `Sized`, если его размер известен во время compilation. Иными словами, это не DST.

### Marker traits

`Sized` — первый пример **marker trait**.\
Marker trait не требует implementation каких-либо methods и не определяет никакого behavior.
Он лишь **помечает** type как обладающий определёнными свойствами.
Затем compiler использует эту отметку, чтобы включить определённое behavior или optimizations.

### Auto traits

Кроме того, `Sized` является **auto trait**.\
Его не нужно реализовывать явно: compiler делает это автоматически
на основе определения type.

### Примеры

Все types, встречавшиеся до сих пор, являются `Sized`: `u32`, `String`, `bool` и т. д.

Как мы только что выяснили, `str` не является `Sized`.\
А вот `&str` является `Sized`! Его размер известен во время compilation: два `usize`, один для pointer,
второй для длины.
