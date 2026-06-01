# Введение

В предыдущей главе мы моделировали `Ticket` изолированно: определили его fields и их constraints, узнали,
как лучше всего представить их в Rust, но не рассматривали место `Ticket` в более крупной системе.
В этой главе мы построим простой workflow вокруг `Ticket`: создадим базовую систему управления для
хранения и получения тикетов.

Задача даст нам возможность изучить новые концепции Rust:

- Arrays, размещаемые в stack
- `Vec`, type расширяемого array
- `Iterator` и `IntoIterator` для iteration по collections
- Slices (`&[T]`) для работы с частями collection
- Lifetimes для описания периода валидности references
- `HashMap` и `BTreeMap`, две data structures типа key-value
- `Eq` и `Hash` для сравнения keys в `HashMap`
- `Ord` и `PartialOrd` для работы с `BTreeMap`
- `Index` и `IndexMut` для доступа к элементам collection
