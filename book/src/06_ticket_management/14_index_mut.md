# Mutable indexing

`Index` предоставляет доступ только для чтения. Он не позволяет изменять
полученное значение.

## `IndexMut`

Чтобы разрешить mutability, необходимо реализовать trait `IndexMut`.

```rust
// Слегка упрощено
pub trait IndexMut<Idx>: Index<Idx>
{
    // Обязательный метод
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output;
}
```

`IndexMut` можно реализовать только для type, уже реализующего `Index`,
поскольку он открывает _дополнительную_ возможность.
