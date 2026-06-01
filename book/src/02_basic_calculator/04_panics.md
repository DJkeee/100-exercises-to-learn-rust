# Panics

Вернёмся к function `speed`, которую вы написали для раздела ["Variables"](02_variables.md).
Вероятно, она выглядела примерно так:

```rust
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    let distance = end - start;
    distance / time_elapsed
}
```

Возможно, вы уже заметили одну проблему[^one]: что произойдёт, если `time_elapsed` равен нулю?

Можете проверить это
[в Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=36e5ddbe3b3f741dfa9f74c956622bac)!\
Программа завершится со следующим сообщением об ошибке:

```text
thread 'main' panicked at src/main.rs:3:5:
attempt to divide by zero
```

Это называется **panic**.\
С помощью panic Rust сообщает, что произошла настолько серьёзная ошибка,
что программа не может продолжать выполнение: это **unrecoverable error**[^catching]. Деление на ноль относится именно к таким
ошибкам.

## Macro panic!

Panic можно вызвать намеренно с помощью macro `panic!`[^macro]:

```rust
fn main() {
    panic!("This is a panic!");
    // Приведенная ниже строка никогда не будет выполнена
    let x = 1 + 2;
}
```

В Rust существуют и другие механизмы работы с recoverable errors, которые [мы рассмотрим позже](../05_ticket_v2/06_fallibility.md).
Пока будем использовать panics как грубое, но простое временное решение.

## Дополнительные материалы

- [Документация macro panic!](https://doc.rust-lang.org/std/macro.panic.html)

[^one]: У `speed` есть ещё одна проблема, к которой мы скоро вернёмся. Сможете её заметить?

[^catching]: Можно попытаться catch panic, но это крайняя мера для строго определённых
ситуаций.

[^macro]: Если за именем следует `!`, это macro invocation. Пока считайте macros более мощными functions. Мы
подробнее рассмотрим их позже в курсе.
