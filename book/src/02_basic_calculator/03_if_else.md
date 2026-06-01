# Control flow, часть 1

До сих пор все наши программы были довольно прямолинейными.\
Последовательность инструкций выполнялась сверху вниз, и на этом всё.

Пришло время познакомиться с **branching**.

## Ветви `if`

Keyword `if` используется для выполнения code block только при истинном условии.

Вот простой пример:

```rust
let number = 3;
if number < 5 {
    println!("`number` is smaller than 5");
}
```

Эта программа выведет `number is smaller than 5`, потому что условие `number < 5` истинно.

### Ветви `else`

Как и большинство языков программирования, Rust поддерживает необязательную ветвь `else`, которая выполняет code block, если условие
expression `if` ложно.\
Например:

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else {
    println!("`number` is greater than or equal to 5");
}
```

### Ветви `else if`

Если несколько expressions `if` вложены друг в друга, code всё сильнее смещается вправо.

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else {
    if number >= 3 {
        println!("`number` is greater than or equal to 3, but smaller than 5");
    } else {
        println!("`number` is smaller than 3");
    }
}
```

С помощью keywords `else if` несколько expressions `if` можно объединить в одно:

```rust
let number = 3;

if number < 5 {
    println!("`number` is smaller than 5");
} else if number >= 3 {
    println!("`number` is greater than or equal to 3, but smaller than 5");
} else {
    println!("`number` is smaller than 3");
}
```

## Booleans

Условие в expression `if` должно иметь type `bool`, то есть быть **boolean**.\
Booleans, как и integers, являются primitive type в Rust.

Boolean может принимать одно из двух значений: `true` или `false`.

### Без значений truthy и falsy

Если условие в expression `if` не является boolean, возникнет compilation error.

Например, следующий code не будет compile:

```rust
let number = 3;
if number {
    println!("`number` is not zero");
}
```

Вы получите следующий compilation error:

```text
error[E0308]: mismatched types
 --> src/main.rs:3:8
  |
3 |     if number {
  |        ^^^^^^ expected `bool`, found integer
```

Это следует из подхода Rust к type coercion: автоматического преобразования не-boolean types в booleans не существует.
В Rust нет концепции значений **truthy** или **falsy**, как в JavaScript или Python.\
Проверяемое условие необходимо формулировать явно.

### Comparison operators

Comparison operators часто используются для построения условий expressions `if`.\
При работе с integers в Rust доступны следующие comparison operators:

- `==`: равно
- `!=`: не равно
- `<`: меньше
- `>`: больше
- `<=`: меньше или равно
- `>=`: больше или равно

## `if/else` является expression

В Rust конструкции `if` являются **expressions**, а не statements: они возвращают значение.\
Это значение можно присвоить variable или использовать в других expressions. Например:

```rust
let number = 3;
let message = if number < 5 {
    "smaller than 5"
} else {
    "greater than or equal to 5"
};
```

В примере выше каждая ветвь `if` вычисляется в string literal,
который затем присваивается variable `message`.\
Единственное требование: обе ветви `if` должны возвращать одинаковый type.
