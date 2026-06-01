# `HashMap`

Наша implementation `Index`/`IndexMut` неидеальна: чтобы получить тикет по id, необходимо выполнить iteration по всему
`Vec`; algorithmic complexity составляет `O(n)`, где
`n` — количество тикетов в store.

Можно добиться лучшего результата, используя для хранения тикетов другую data structure: `HashMap<K, V>`.

```rust
use std::collections::HashMap;

// Вывод типов позволяет опустить явную сигнатуру типа (в этом
// примере это был бы `HashMap<String, String>`).
let mut book_reviews = HashMap::new();

book_reviews.insert(
    "Adventures of Huckleberry Finn".to_string(),
    "My favorite book.".to_string(),
);
```

`HashMap` работает с парами key-value. Он является generic по обоим типам: `K` — generic
parameter для type key, а `V` — для type value.

Ожидаемая стоимость вставки, получения и удаления **постоянна**: `O(1)`.
Звучит идеально для нашего use case, не так ли?

## Требования к key

В определении struct `HashMap` нет trait bounds, но они встречаются
у его методов. Рассмотрим, например, `insert`:

```rust
// Слегка упрощено
impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        // [...]
    }
}
```

Type key должен реализовывать traits `Eq` и `Hash`.\
Разберём их подробнее.

## `Hash`

Hash function, или hasher, отображает потенциально бесконечное множество значений, например
всех возможных строк, в ограниченный диапазон, например в значения `u64`.\
Существует множество различных hash functions с разными свойствами:
скоростью, риском collision, обратимостью и т. д.

Как следует из названия, внутри `HashMap` используется hash function.
Он вычисляет hash для key, а затем использует этот hash для сохранения или получения связанного значения.
При такой стратегии type key должен быть hashable, поэтому для `K` задан trait bound `Hash`.

Trait `Hash` находится в module `std::hash`:

```rust
pub trait Hash {
    // Обязательный метод
    fn hash<H>(&self, state: &mut H)
       where H: Hasher;
}
```

Вручную реализовывать `Hash` приходится редко. Обычно используется derive:

```rust
#[derive(Hash)]
struct Person {
    id: u32,
    name: String,
}
```

## `Eq`

`HashMap` должен уметь сравнивать keys на равенство. Это особенно важно
при hash collisions, то есть когда два разных key дают одинаковый hash.

Может возникнуть вопрос: разве для этого не предназначен trait `PartialEq`? Почти!\
Для `HashMap` недостаточно `PartialEq`, поскольку он не гарантирует reflexivity, то есть что `a == a` всегда имеет значение `true`.\
Например, числа с floating point (`f32` и `f64`) реализуют `PartialEq`,
но не обладают свойством reflexivity: `f32::NAN == f32::NAN` имеет значение `false`.\
Reflexivity критически важна для корректной работы `HashMap`: без неё не удалось бы получить значение
из map с помощью того же key, с которым оно было вставлено.

Trait `Eq` расширяет `PartialEq` свойством reflexivity:

```rust
pub trait Eq: PartialEq {
    // Дополнительных методов нет
}
```

Это marker trait: он не добавляет новых методов, а лишь позволяет сообщить compiler,
что логика равенства, реализованная в `PartialEq`, обладает reflexivity.

При derive `PartialEq` можно автоматически добавить derive `Eq`:

```rust
#[derive(PartialEq, Eq)]
struct Person {
    id: u32,
    name: String,
}
```

## Связь `Eq` и `Hash`

Между `Eq` и `Hash` действует неявный контракт: если два key равны, их hashes также должны быть равны.
Это критически важно для корректной работы `HashMap`. Если нарушить этот контракт, использование `HashMap`
будет давать бессмысленные результаты.
