# Indexing

`TicketStore::get` возвращает `Option<&Ticket>` для заданного `TicketId`.\
Ранее мы уже видели, как обращаться к элементам arrays и vectors с помощью
синтаксиса indexing в Rust:

```rust
let v = vec![0, 1, 2];
assert_eq!(v[0], 0);
```

Как предоставить аналогичную возможность для `TicketStore`?\
Как вы уже догадались, нужно реализовать trait `Index`!

## `Index`

Trait `Index` определён в standard library Rust:

```rust
// Слегка упрощено
pub trait Index<Idx>
{
    type Output;

    // Обязательный метод
    fn index(&self, index: Idx) -> &Self::Output;
}
```

У него есть:

- Один generic parameter `Idx`, представляющий type index
- Один associated type `Output`, представляющий type, получаемый с помощью index

Обратите внимание: метод `index` не возвращает `Option`. Предполагается, что
`index` вызовет panic при попытке обратиться к отсутствующему элементу, как и при
indexing array или vec.
