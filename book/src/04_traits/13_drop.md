# Trait `Drop`

Когда мы знакомились с [destructors](../03_ticket_v1/11_destructor.md),
то упомянули, что function `drop`:

1. освобождает memory, занимаемую type (то есть `std::mem::size_of` bytes)
2. очищает дополнительные ресурсы, которыми может управлять value (например, buffer `String` в heap)

На шаге 2 в дело вступает trait `Drop`.

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

Trait `Drop` позволяет определить для своих types _дополнительную_ логику очистки
помимо той, которую compiler выполняет автоматически.\
Всё, что находится в method `drop`, будет выполнено при выходе value из scope.

## `Drop` и `Copy`

Обсуждая trait `Copy`, мы сказали, что type не может реализовывать `Copy`, если он
управляет дополнительными ресурсами помимо занимаемых им в memory `std::mem::size_of` bytes.

Возникает вопрос: как compiler узнаёт, управляет ли type дополнительными ресурсами?
Верно: по implementations trait `Drop`!\
Если у type есть явная implementation `Drop`, compiler предполагает,
что с type связаны дополнительные ресурсы, и не разрешает реализовать `Copy`.

```rust
// Это единичная структура, то есть структура без полей.
#[derive(Clone, Copy)]
struct MyType;

impl Drop for MyType {
    fn drop(&mut self) {
       // Здесь ничего делать не нужно,
       // достаточно «пустой» реализации Drop
    }
}
```

Compiler сообщит следующую ошибку:

```text
error[E0184]: the trait `Copy` cannot be implemented for this type; 
              the type has a destructor
 --> src/lib.rs:2:17
  |
2 | #[derive(Clone, Copy)]
  |                 ^^^^ `Copy` not allowed on types with destructors
```
