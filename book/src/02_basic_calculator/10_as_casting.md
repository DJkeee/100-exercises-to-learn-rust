# Conversions, часть 1

Мы уже не раз повторяли, что Rust не выполняет
implicit type conversions для integers.\
Как же выполнять _explicit_ conversions?

## `as`

Для преобразования между integer types можно использовать operator `as`.\
Conversions с `as` являются **infallible**.

Например:

```rust
let a: u32 = 10;

// Преобразуем `a` в тип `u64`
let b = a as u64;

// В качестве целевого типа можно использовать `_`,
// если компилятор может правильно вывести тип.
// Например:
let c: u64 = a as _;
```

Semantics этого conversion вполне ожидаема: все значения `u32` являются допустимыми значениями `u64`.

### Truncation

В обратном направлении всё становится интереснее:

```rust
// Число, слишком большое,
// чтобы поместиться в `u8`
let a: u16 = 255 + 1;
let b = a as u8;
```

Эта программа выполнится без проблем, поскольку conversions с `as` являются infallible.
Но чему равно значение `b`?
При переходе от более крупного integer type к меньшему compiler Rust выполнит
**truncation**.

Чтобы понять происходящее, сначала посмотрим, как `256u16`
представлено в memory в виде последовательности bits:

```text
 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0
|               |               |
+---------------+---------------+
  First 8 bits    Last 8 bits
```

При conversion в `u8` compiler Rust сохранит последние 8 bits представления `u16`
в memory:

```text
 0 0 0 0 0 0 0 0 
|               |
+---------------+
  Last 8 bits
```

Таким образом, `256 as u8` равно `0`. В большинстве случаев это... не лучший результат.\
Более того, compiler Rust постарается остановить вас, если заметит попытку
выполнить cast literal value, приводящий к truncation:

```text
error: literal out of range for `i8`
  |
4 |     let a = 255 as i8;
  |             ^^^
  |
  = note: the literal `255` does not fit into the type `i8` 
          whose range is `-128..=127`
  = help: consider using the type `u8` instead
  = note: `#[deny(overflowing_literals)]` on by default
```

### Рекомендация

Общее правило: соблюдайте осторожность при casting с помощью `as`.\
Используйте его _исключительно_ для перехода от меньшего type к большему.
Для перехода от большего integer type к меньшему полагайтесь на
[механизм _fallible_ conversion](../05_ticket_v2/13_try_from.md), который мы
рассмотрим позже в курсе.

### Ограничения

Неожиданное поведение — не единственный недостаток casting с помощью `as`.
Его возможности также весьма ограничены: casting с помощью `as`
можно использовать только для primitive types и нескольких других особых случаев.\
При работе с composite types потребуется применять
другие механизмы conversion ([fallible](../05_ticket_v2/13_try_from.md)
и [infallible](../04_traits/09_from.md)), которые мы рассмотрим позже.

## Дополнительные материалы

- Прочитайте [официальный reference Rust](https://doc.rust-lang.org/reference/expressions/operator-expr.html#numeric-cast),
  чтобы узнать точное поведение casting с помощью `as` для каждой комбинации source/target,
  а также полный список разрешённых conversions.
