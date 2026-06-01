# Loops, часть 1: `while`

В implementation `factorial` вам пришлось использовать recursion.\
Это может казаться естественным, особенно если вы знакомы с functional programming.
Или непривычным, если вы привыкли к более imperative языкам, таким как C или Python.

Посмотрим, как реализовать ту же функциональность с помощью **loop**.

## Loop `while`

Loop `while` позволяет выполнять code block, пока **условие** истинно.\
Вот общий syntax:

```rust
while <condition> {
    // выполняемый код
}
```

Например, можно просуммировать числа от 1 до 5:

```rust
let sum = 0;
let i = 1;
// «пока i меньше или равно 5»
while i <= 5 {
    // `+=` — сокращенная запись для `sum = sum + i`
    sum += i;
    i += 1;
}
```

Программа будет прибавлять 1 к `i`, а `i` — к `sum`, пока `i` не станет больше 5.

## Keyword `mut`

Приведённый выше пример в текущем виде не будет compile. Вы получите такую ошибку:

```text
error[E0384]: cannot assign twice to immutable variable `sum`
 --> src/main.rs:7:9
  |
2 |     let sum = 0;
  |         ---
  |         |
  |         first assignment to `sum`
  |         help: consider making this binding mutable: `mut sum`
...
7 |         sum += i;
  |         ^^^^^^^^ cannot assign twice to immutable variable

error[E0384]: cannot assign twice to immutable variable `i`
 --> src/main.rs:8:9
  |
3 |     let i = 1;
  |         -
  |         |
  |         first assignment to `i`
  |         help: consider making this binding mutable: `mut i`
...
8 |         i += 1;
  |         ^^^^^^ cannot assign twice to immutable variable
```

Причина в том, что variables в Rust по умолчанию **immutable**.\
После присваивания их значение нельзя изменить.

Чтобы разрешить изменения, variable нужно объявить как **mutable** с помощью keyword `mut`:

```rust
// Теперь `sum` и `i` изменяемы!
let mut sum = 0;
let mut i = 1;

while i <= 5 {
    sum += i;
    i += 1;
}
```

Теперь code будет compile и выполнится без ошибок.

## Дополнительные материалы

- [Документация loop `while`](https://doc.rust-lang.org/std/keyword.while.html)
