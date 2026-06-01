# Vectors

Сильная сторона arrays одновременно является и их слабостью: размер должен быть известен заранее, во время компиляции.
Если попытаться создать array, размер которого становится известен только во время выполнения, возникнет ошибка компиляции:

```rust
let n = 10;
let numbers: [u32; n];
```

```text
error[E0435]: attempt to use a non-constant value in a constant
 --> src/main.rs:3:20
  |
2 | let n = 10;
3 | let numbers: [u32; n];
  |                    ^ non-constant value
```

Arrays не подойдут для нашей системы управления тикетами: во время компиляции мы не знаем, сколько тикетов потребуется хранить.
Здесь нам пригодится `Vec`.

## `Vec`

`Vec` — type расширяемого array из standard library.\
Создать пустой array можно с помощью функции `Vec::new`:

```rust
let mut numbers: Vec<u32> = Vec::new();
```

Затем элементы можно добавлять в vector с помощью метода `push`:

```rust
numbers.push(1);
numbers.push(2);
numbers.push(3);
```

Новые значения добавляются в конец vector.\
Если значения известны в момент создания, можно также создать инициализированный vector с помощью macro `vec!`:

```rust
let numbers = vec![1, 2, 3];
```

## Доступ к элементам

Синтаксис доступа к элементам такой же, как у arrays:

```rust
let numbers = vec![1, 2, 3];
let first = numbers[0];
let second = numbers[1];
let third = numbers[2];
```

Index должен иметь type `usize`.\
Можно также использовать метод `get`, который возвращает `Option<&T>`:

```rust
let numbers = vec![1, 2, 3];
assert_eq!(numbers.get(0), Some(&1));
// При попытке доступа по индексу за границами возвращается `None`,
// а не возникает паника.
assert_eq!(numbers.get(3), None);
```

Как и при доступе к элементам arrays, здесь выполняется bounds checking. Сложность операции — O(1).

## Размещение в памяти

`Vec` — data structure, размещаемая в heap.\
При создании `Vec` память для хранения элементов выделяется в heap.

Если выполнить следующий код:

```rust
let mut numbers = Vec::with_capacity(3);
numbers.push(1);
numbers.push(2);
```

получится следующая схема размещения в памяти:

```text
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   2    |    3     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+
Heap:  | 1 | 2 | ? |
       +---+---+---+
```

`Vec` отслеживает три значения:

- **pointer** на зарезервированную область heap.
- **length** vector, то есть количество элементов в нём.
- **capacity** vector, то есть количество элементов, которые могут поместиться в зарезервированной области heap.

Эта схема должна показаться знакомой: она в точности совпадает со схемой `String`!\
Это не случайность: внутри `String` определён как vector bytes, `Vec<u8>`:

```rust
pub struct String {
    vec: Vec<u8>,
}
```
