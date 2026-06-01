# Syntax

<div class="warning">

Не забегайте вперёд!\
Прежде чем начать этот раздел, выполните упражнение из предыдущего.\
Оно находится в `exercises/01_intro/00_welcome`, в [GitHub repository курса](https://github.com/mainmatter/100-exercises-to-learn-rust).\
Используйте [`wr`](00_welcome.md#wr-the-workshop-runner), чтобы начать курс и проверить свои решения.

</div>

Предыдущее задание даже нельзя назвать полноценным упражнением, но оно уже познакомило вас со многими элементами **syntax** Rust.
Мы не будем рассматривать каждую деталь syntax Rust, использованную в предыдущем упражнении.
Вместо этого разберём _ровно столько_, сколько нужно, чтобы двигаться дальше и не увязнуть в деталях.\
Шаг за шагом!

## Comments

Для однострочных comments можно использовать `//`:

```rust
// Это однострочный комментарий
// За ним следует еще один однострочный комментарий
```

## Functions

Functions в Rust определяются при помощи keyword `fn`, после которого указываются имя function, её input parameters и
return type.
Тело function заключается в фигурные скобки `{}`.

В предыдущем упражнении вы видели function `greeting`:

```rust
// `fn` <function_name> ( <input params> ) -> <return_type> { <body> }
fn greeting() -> &'static str {
    // TODO: исправьте меня 👇
    "I'm ready to __!"
}
```

`greeting` не имеет input parameters и возвращает reference на string slice (`&'static str`).

### Return type

Return type можно опустить в signature, если function ничего не возвращает (то есть возвращает `()`,
unit type Rust).
Именно так оформлена function `test_welcome`:

```rust
fn test_welcome() {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
```

Приведённый выше код эквивалентен следующему:

```rust
// Явно указываем единичный возвращаемый тип
//                   👇
fn test_welcome() -> () {
    assert_eq!(greeting(), "I'm ready to learn Rust!");
}
```

### Возврат значений

Последнее expression в function возвращается неявно:

```rust
fn greeting() -> &'static str {
    // Это последнее выражение в функции
    // Поэтому его значение возвращается из `greeting`
    "I'm ready to learn Rust!"
}
```

Для досрочного возврата значения также можно использовать keyword `return`:

```rust
fn greeting() -> &'static str {
    // Обратите внимание на точку с запятой в конце строки!
    return "I'm ready to learn Rust!";
}
```

Если возможно, keyword `return` принято опускать: это считается idiomatic подходом.

### Input parameters

Input parameters объявляются внутри круглых скобок `()`, следующих за именем function.\
Для каждого parameter указывается его имя, затем двоеточие `:`, затем type.

Например, приведённая ниже function `greet` принимает parameter `name` типа `&str` (`string slice`):

```rust
// Входной параметр
//        👇
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

Если input parameters несколько, их необходимо разделять запятыми.

### Type annotations

Поскольку мы уже несколько раз упомянули types, сформулируем явно: Rust является **statically typed language**.\
Каждое значение в Rust имеет type, который должен быть известен compiler на этапе compile-time.

Types являются одной из форм **static analysis**.\
Можно считать type **меткой**, которую compiler прикрепляет к каждому значению в программе. В зависимости от этой
метки compiler может применять разные правила: например, нельзя сложить строку с числом, но можно сложить два числа.
При правильном использовании types предотвращают целые классы runtime bugs.
