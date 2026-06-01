# Поведение для отдельных случаев

`overflow-checks` — довольно грубый инструмент: это global setting, влияющий на всю программу.\
Часто integer overflows требуется обрабатывать по-разному в зависимости от контекста: иногда
правильным выбором будет wrapping, а иногда предпочтительнее panic.

## `wrapping_` methods

Wrapping arithmetic можно включать для отдельных operations с помощью methods `wrapping_`[^method].\
Например, `wrapping_add` позволяет сложить два integers с wrapping:

```rust
let x = 255u8;
let y = 1u8;
let sum = x.wrapping_add(y);
assert_eq!(sum, 0);
```

## Methods `saturating_`

Вместо этого с помощью methods `saturating_` можно выбрать **saturating arithmetic**.\
При saturating arithmetic возвращается максимальное или минимальное значение integer type, а wrapping не выполняется.
Например:

```rust
let x = 255u8;
let y = 1u8;
let sum = x.saturating_add(y);
assert_eq!(sum, 255);
```

Поскольку `255 + 1` равно `256`, что больше `u8::MAX`, результатом будет `u8::MAX` (255).\
Для underflows происходит обратное: `0 - 1` равно `-1`, что меньше `u8::MIN`, поэтому результатом будет `u8::MIN` (0).

Saturating arithmetic нельзя включить через setting profile `overflow-checks`: её нужно выбирать явно
при выполнении arithmetic operation.

[^method]: Methods можно считать functions, «прикреплёнными» к определённому type.
Methods и способы их определения мы рассмотрим в следующей главе.
