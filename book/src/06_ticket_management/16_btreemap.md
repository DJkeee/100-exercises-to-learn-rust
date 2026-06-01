# Ordering

Перейдя с `Vec` на `HashMap`, мы повысили производительность системы управления тикетами
и заодно упростили код.\
Однако есть и недостаток. При iteration по store на основе `Vec` можно было быть уверенными, что тикеты
будут возвращены в порядке добавления.\
С `HashMap` это не так: iteration по тикетам возможна, но порядок будет случайным.

Вернуть стабильный ordering можно, перейдя с `HashMap` на `BTreeMap`.

## `BTreeMap`

`BTreeMap` гарантирует сортировку entries по keys.\
Это полезно, когда требуется выполнить iteration по entries в определённом порядке или
range queries, например «получить все тикеты с id от 10 до 20».

Как и в случае `HashMap`, в определении `BTreeMap` нет trait bounds.
Однако они есть у его методов. Рассмотрим `insert`:

```rust
// `K` и `V` обозначают типы ключа и значения соответственно,
// как и в `HashMap`.
impl<K, V> BTreeMap<K, V> {
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        // реализация
    }
}
```

`Hash` больше не требуется. Вместо него type key должен реализовывать trait `Ord`.

## `Ord`

Trait `Ord` используется для сравнения значений.\
Если `PartialEq` используется для сравнения на равенство, то `Ord` — для сравнения по ordering.

Он определён в `std::cmp`:

```rust
pub trait Ord: Eq + PartialOrd {
    fn cmp(&self, other: &Self) -> Ordering;
}
```

Метод `cmp` возвращает enum `Ordering`, который может принимать одно
из значений: `Less`, `Equal` или `Greater`.\
`Ord` требует реализации двух других traits: `Eq` и `PartialOrd`.

## `PartialOrd`

`PartialOrd` — более слабая версия `Ord`, как и `PartialEq` — более слабая версия `Eq`.
Это видно из его определения:

```rust
pub trait PartialOrd: PartialEq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}
```

`PartialOrd::partial_cmp` возвращает `Option`: возможность сравнить два значения
не гарантируется.\
Например, `f32` не реализует `Ord`, поскольку значения `NaN` нельзя сравнивать.
По той же причине `f32` не реализует `Eq`.

## Реализация `Ord` и `PartialOrd`

Для собственных типов можно выполнить derive как `Ord`, так и `PartialOrd`:

```rust
// Также нужно добавить `Eq` и `PartialEq`,
// поскольку они требуются для `Ord`.
#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct TicketId(u64);
```

Если вы решили или вынуждены реализовать их вручную, будьте внимательны:

- `Ord` и `PartialOrd` должны быть согласованы с `Eq` и `PartialEq`.
- `Ord` и `PartialOrd` должны быть согласованы друг с другом.
