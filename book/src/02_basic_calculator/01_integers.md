# Types, часть 1

В разделе ["Syntax"](../01_intro/01_syntax.md) input parameters `compute` имели type `u32`.\
Разберёмся, что это _означает_.

## Primitive types

`u32` является одним из **primitive types** Rust. Primitive types — это простейшие строительные блоки языка.
Они встроены в сам язык, то есть не определяются через другие types.

Primitive types можно комбинировать, создавая более сложные types. Скоро мы увидим, как это делается.

## Integers

В частности, `u32` — это **unsigned 32-bit integer**.

Integer — это число, которое можно записать без дробной части. Например, `1` является integer, а `1.2` — нет.

### Signed и unsigned

Integer может быть **signed** или **unsigned**.\
Unsigned integer может представлять только неотрицательные числа (то есть `0` или больше).
Signed integer может представлять как положительные, так и отрицательные числа (например, `-1`, `12` и т. д.).

Буква `u` в `u32` означает **unsigned**.\
Соответствующий type для signed integer — `i32`, где `i` означает integer (то есть любое целое число: положительное или
отрицательное).

### Bit width

Число `32` в `u32` обозначает **количество bits[^bit]**, используемых для представления числа в memory.\
Чем больше bits, тем шире диапазон представимых чисел.

Rust поддерживает несколько вариантов bit width для integers: `8`, `16`, `32`, `64`, `128`.

При 32 bits `u32` может представлять числа от `0` до `2^32 - 1` (то есть до [`u32::MAX`](https://doc.rust-lang.org/std/primitive.u32.html#associatedconstant.MAX)).\
При том же количестве bits signed integer (`i32`) может представлять числа от `-2^31` до `2^31 - 1`
(то есть от [`i32::MIN`](https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MIN)
до [`i32::MAX`](https://doc.rust-lang.org/std/primitive.i32.html#associatedconstant.MAX)).\
Максимальное значение `i32` меньше максимального значения `u32`, поскольку один bit используется для представления
знака числа. Подробнее о представлении signed integers в memory можно прочитать о
[two's complement](https://en.wikipedia.org/wiki/Two%27s_complement).

### Итоги

Комбинируя два параметра (signed/unsigned и bit width), получаем следующие integer types:

| Bit width | Signed | Unsigned |
| --------- | ------ | -------- |
| 8-bit     | `i8`   | `u8`     |
| 16-bit    | `i16`  | `u16`    |
| 32-bit    | `i32`  | `u32`    |
| 64-bit    | `i64`  | `u64`    |
| 128-bit   | `i128` | `u128`   |

## Literals

**Literal** — это запись фиксированного значения в source code.\
Например, `42` является literal Rust для числа сорок два.

### Type annotations для literals

Но у каждого значения в Rust есть type. Тогда какой type у `42`?

Compiler Rust попытается вывести type literal из контекста его использования.\
Если контекст отсутствует, compiler по умолчанию назначит integer literals type `i32`.\
Если нужен другой type, желаемый integer type можно добавить в виде suffix: например, `2u64` — это число 2,
для которого явно указан type `u64`.

### Символы подчёркивания в literals

Для удобства чтения больших чисел можно использовать символы подчёркивания `_`.\
Например, `1_000_000` эквивалентно `1000000`.

## Arithmetic operators

Rust поддерживает следующие arithmetic operators[^traits] для integers:

- `+` для сложения
- `-` для вычитания
- `*` для умножения
- `/` для деления
- `%` для остатка от деления

Правила precedence и associativity для этих operators совпадают с математическими.\
Чтобы изменить precedence по умолчанию, можно использовать скобки. Например, `2 * (3 + 4)`.

> ⚠️ **Предупреждение**
>
> При использовании с integer types operator деления `/` выполняет integer division.
> Иными словами, результат округляется к нулю. Например, `5 / 2` равно `2`, а не `2.5`.

## Без автоматического type coercion

Как мы уже обсуждали в предыдущем упражнении, Rust является statically typed language.\
В частности, Rust довольно строго относится к type coercion. Он не станет автоматически преобразовывать значение одного type
в другой[^coercion],
даже если преобразование не приводит к потере данных. Это необходимо делать явно.

Например, значение `u8` нельзя присвоить variable типа `u32`, хотя все значения `u8` являются допустимыми значениями `u32`:

```rust
let b: u8 = 100;
let a: u32 = b;
```

Это приведёт к compilation error:

```text
error[E0308]: mismatched types
  |
3 |     let a: u32 = b;
  |            ---   ^ expected `u32`, found `u8`
  |            |
  |            expected due to this
  |
```

Позже в курсе мы рассмотрим [преобразование между types](../04_traits/09_from.md).

## Дополнительные материалы

- [Раздел об integer types](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types) в официальной книге по Rust

[^bit]: Bit — это минимальная единица данных в компьютере. Он может принимать только два значения: `0` или `1`.

[^traits]: Rust не позволяет определять собственные operators, но даёт возможность управлять поведением встроенных
operators.
Мы поговорим об operator overloading [позже в курсе](../04_traits/03_operator_overloading.md), после изучения traits.

[^coercion]: У этого правила есть несколько исключений, в основном связанных с references, smart pointers и ergonomics. Мы
рассмотрим их [позже](../04_traits/07_deref.md).
Пока полезно придерживаться мысленной модели «все преобразования выполняются явно».
