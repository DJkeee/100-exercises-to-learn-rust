# Combinators

Возможности iterators не ограничиваются циклами `for`!\
В документации trait `Iterator` вы найдёте **огромный** набор методов,
позволяющих различными способами преобразовывать, фильтровать и объединять iterators.

Перечислим наиболее распространённые:

- `map` применяет функцию к каждому элементу iterator.
- `filter` сохраняет только элементы, удовлетворяющие predicate.
- `filter_map` объединяет `filter` и `map` в один шаг.
- `cloned` преобразует iterator references в iterator значений, клонируя каждый элемент.
- `enumerate` возвращает новый iterator, выдающий пары `(index, value)`.
- `skip` пропускает первые `n` элементов iterator.
- `take` останавливает iterator после `n` элементов.
- `chain` объединяет два iterator в один.

Эти методы называются **combinators**.\
Обычно из них составляют **chain**, чтобы лаконично и понятно описать сложные преобразования:

```rust
let numbers = vec![1, 2, 3, 4, 5];
// Сумма квадратов четных чисел
let outcome: u32 = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .sum();
```

## Closures

Что происходит с приведёнными выше методами `filter` и `map`?\
Они принимают **closures** в качестве аргументов.

Closures — это **anonymous functions**, то есть функции, определённые без привычного синтаксиса `fn`.\
Они задаются с помощью синтаксиса `|args| body`, где `args` — аргументы, а `body` — тело функции.
`body` может быть блоком кода или одним expression.
Например:

```rust
// Анонимная функция, прибавляющая 1 к аргументу
let add_one = |x| x + 1;
// Также можно записать с помощью блока:
let add_one = |x| { x + 1 };
```

Closures могут принимать несколько аргументов:

```rust
let add = |x, y| x + y;
let sum = add(1, 2);
```

Они также могут выполнять capture переменных из своего environment:

```rust
let x = 42;
let add_x = |y| x + y;
let sum = add_x(1);
```

При необходимости можно указать типы аргументов и/или возвращаемый тип:

```rust
// Только входной тип
let add_one = |x: i32| x + 1;
// Или входной и выходной типы с использованием синтаксиса `fn`
let add_one: fn(i32) -> i32 = |x| x + 1;
```

## `collect`

Что делать после завершения преобразований iterator с помощью combinators?\
Можно выполнить iteration по преобразованным значениям с помощью цикла `for` или собрать их в коллекцию.

Для второго варианта используется метод `collect`.\
`collect` consumes iterator и собирает его элементы в выбранную коллекцию.

Например, квадраты чётных чисел можно собрать в `Vec`:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let squares_of_evens: Vec<u32> = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    .collect();
```

`collect` является generic по **return type**.\
Поэтому обычно требуется предоставить type hint, чтобы помочь compiler вывести правильный тип.
В приведённом выше примере мы указали для `squares_of_evens` тип `Vec<u32>`.
В качестве альтернативы тип можно указать с помощью **turbofish syntax**:

```rust
let squares_of_evens = numbers.iter()
    .filter(|&n| n % 2 == 0)
    .map(|&n| n * n)
    // Синтаксис turbofish: `<method_name>::<type>()`
    // Он называется turbofish, потому что `::<>` похож на рыбу
    .collect::<Vec<u32>>();
```

## Дополнительные материалы

- [Документация `Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) содержит
  обзор методов, доступных для iterators в `std`.
- [Crate `itertools`](https://docs.rs/itertools/) определяет **ещё больше** combinators для iterators.
