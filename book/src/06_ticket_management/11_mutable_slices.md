# Mutable slices

До сих пор при обсуждении типов slice, таких как `str` и `[T]`, мы использовали их форму immutable borrow: `&str` и `&[T]`.\
Но slices могут быть и mutable!

Mutable slice создаётся так:

```rust
let mut numbers = vec![1, 2, 3];
let slice: &mut [i32] = &mut numbers;
```

После этого элементы slice можно изменять:

```rust
slice[0] = 42;
```

Первый элемент `Vec` изменится на `42`.

## Ограничения

Для работы с immutable borrows рекомендация была однозначной: предпочитайте slice references вместо references на
owned type, например `&[T]` вместо `&Vec<T>`.\
Для mutable borrows это **не** так.

Рассмотрим следующий пример:

```rust
let mut numbers = Vec::with_capacity(2);
let mut slice: &mut [i32] = &mut numbers;
slice.push(1);
```

Этот код не скомпилируется!\
`push` — метод `Vec`, а не slices. Это проявление более общего принципа: Rust не позволяет
добавлять элементы в slice или удалять их из него. Можно лишь изменять или заменять уже
существующие элементы.

В этом отношении `&mut Vec` и `&mut String` предоставляют строго больше возможностей, чем `&mut [T]` и `&mut str`.\
Выбирайте type, который лучше всего соответствует необходимым операциям.
