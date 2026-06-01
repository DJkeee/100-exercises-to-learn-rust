# Variables

В Rust для объявления **variables** используется keyword `let`.\
Например:

```rust
let x = 42;
```

Выше мы определили variable `x` и присвоили ей значение `42`.

## Type

Каждая variable в Rust должна иметь type. Compiler может вывести его самостоятельно, либо developer может указать его
явно.

### Явная type annotation

Чтобы указать type variable, добавьте после имени variable двоеточие `:`, а затем type. Например:

```rust
// let <variable_name>: <type> = <expression>;
let x: u32 = 42;
```

В примере выше мы явно ограничили type `x` значением `u32`.

### Type inference

Если type variable не указан, compiler попытается вывести его из контекста использования variable.

```rust
let x = 42;
let y: u32 = x;
```

В примере выше мы не указали type `x`.\
Позже `x` присваивается `y`, для которой явно указан type `u32`. Поскольку Rust не выполняет автоматический type coercion,
compiler выводит для `x` type `u32`: такой же, как у `y`, и единственный, при котором программа будет compile
без ошибок.

### Ограничения inference

Иногда compiler требуется небольшая помощь, чтобы вывести правильный type variable из её использования.\
В таких случаях вы получите compilation error, а compiler попросит добавить явную type hint, чтобы
устранить неоднозначность.

## Function arguments также являются variables

Не все герои носят плащи, и не все variables объявляются с помощью `let`.\
Function arguments тоже являются variables!

```rust
fn add_one(x: u32) -> u32 {
    x + 1
}
```

В примере выше `x` является variable типа `u32`.\
Единственное различие между `x` и variable, объявленной с помощью `let`, заключается в том, что для function arguments type
**необходимо** указывать явно. Compiler не станет выводить его автоматически.\
Благодаря этому ограничению compiler Rust (и мы, люди!) может понять signature function, не заглядывая
в её implementation. Это значительно ускоряет compilation[^speed]!

## Initialization

Variable не обязательно инициализировать при объявлении.\
Например,

```rust
let x: u32;
```

является корректным объявлением variable.\
Однако variable необходимо инициализировать перед использованием. Иначе compiler выдаст ошибку:

```rust
let x: u32;
let y = x + 1;
```

приведёт к compilation error:

```text
error[E0381]: used binding `x` isn't initialized
 --> src/main.rs:3:9
  |
2 | let x: u32;
  |     - binding declared here but left uninitialized
3 | let y = x + 1;
  |         ^ `x` used here but it isn't initialized
  |
help: consider assigning a value
  |
2 | let x: u32 = 0;
  |            +++
```

[^speed]: Когда речь идёт о скорости compilation, compiler Rust нужна любая возможная помощь.
