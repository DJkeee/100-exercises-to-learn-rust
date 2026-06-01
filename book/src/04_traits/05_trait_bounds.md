# Trait bounds

До сих пор мы рассмотрели два варианта использования traits:

- Открытие «встроенного» behavior (например, operator overloading)
- Добавление нового behavior к существующим types (то есть extension traits)

Есть и третий вариант: **generic programming**.

## Проблема

До сих пор все наши functions и methods работали с **concrete types**.\
Код, работающий с concrete types, обычно легко писать и понимать. Но возможности его
повторного использования ограничены.\
Представим, например, что мы хотим написать function, возвращающую `true`, если целое число чётное.
При работе с concrete types пришлось бы написать отдельную function для каждого целочисленного type, который мы хотим
поддерживать:

```rust
fn is_even_i32(n: i32) -> bool {
    n % 2 == 0
}

fn is_even_i64(n: i64) -> bool {
    n % 2 == 0
}

// И так далее.
```

В качестве альтернативы можно написать один extension trait, а затем создать разные implementations для каждого целочисленного type:

```rust
trait IsEven {
    fn is_even(&self) -> bool;
}

impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEven for i64 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// И так далее.
```

Дублирование никуда не исчезло.

## Generic programming

С помощью **generics** можно добиться большего.\
Generics позволяют писать код, работающий с **type parameter** вместо concrete type:

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
{
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

`print_if_even` — это **generic function**.\
Она не привязана к конкретному input type. Вместо этого она работает с любым type `T`, который:

- Реализует trait `IsEven`.
- Реализует trait `Debug`.

Этот contract выражается через **trait bound**: `T: IsEven + Debug`.\
Символ `+` используется, чтобы потребовать от `T` implementation нескольких traits. `T: IsEven + Debug` эквивалентно
формулировке «где `T` реализует `IsEven` **и** `Debug`».

## Trait bounds

Для чего нужны trait bounds в `print_if_even`?\
Чтобы выяснить это, попробуем их удалить:

```rust
fn print_if_even<T>(n: T) {
    if n.is_even() {
        println!("{n:?} is even");
    }
}
```

Этот код не скомпилируется:

```text
error[E0599]: no method named `is_even` found for type parameter `T` 
              in the current scope
 --> src/lib.rs:2:10
  |
1 | fn print_if_even<T>(n: T) {
  |                  - method `is_even` not found 
  |                    for this type parameter
2 |     if n.is_even() {
  |          ^^^^^^^ method not found in `T`

error[E0277]: `T` doesn't implement `Debug`
 --> src/lib.rs:3:19
  |
3 |         println!("{n:?} is even");
  |                   ^^^^^ 
  |   `T` cannot be formatted using `{:?}` because 
  |         it doesn't implement `Debug`
  |
help: consider restricting type parameter `T`
  |
1 | fn print_if_even<T: std::fmt::Debug>(n: T) {
  |                   +++++++++++++++++
```

Без trait bounds compiler не знает, что `T` **умеет делать**.\
Он не знает, что у `T` есть method `is_even`, и не знает, как форматировать `T` для вывода.
С точки зрения compiler у обычного `T` вообще нет никакого behavior.\
Trait bounds ограничивают набор доступных types, гарантируя наличие behavior, необходимого телу function.

## Синтаксис: inline trait bounds

Во всех примерах выше для указания trait bounds использовалась **`where` clause**:

```rust
fn print_if_even<T>(n: T)
where
    T: IsEven + Debug
//  ^^^^^^^^^^^^^^^^^
//  Это предложение `where`
{
    // [...]
}
```

Если trait bounds просты, их можно разместить **inline** непосредственно рядом с type parameter:

```rust
fn print_if_even<T: IsEven + Debug>(n: T) {
    //           ^^^^^^^^^^^^^^^^^
    //           Это встроенное ограничение трейта
    // [...]
}
```

## Синтаксис: осмысленные имена

В примерах выше мы использовали `T` в качестве имени type parameter. Это распространённое соглашение для functions
с единственным type parameter.\
Однако ничто не мешает выбрать более осмысленное имя:

```rust
fn print_if_even<Number: IsEven + Debug>(n: Number) {
    // [...]
}
```

Использовать осмысленные имена даже **желательно**, если задействовано несколько type parameters или имя
`T` недостаточно ясно передаёт роль type в function.
Выбирая имена type parameters, стремитесь к ясности и читаемости так же, как при именовании variables или function parameters.
При этом соблюдайте соглашения Rust: используйте [upper camel case для имён type parameters](https://rust-lang.github.io/api-guidelines/naming.html#casing-conforms-to-rfc-430-c-case).

## Function signature важнее всего

Возможно, вы задаётесь вопросом, зачем вообще нужны trait bounds. Разве compiler не может вывести необходимые traits из тела function?\
Может, но делать этого не будет.\
Причина та же, что и для [явных type annotations у function parameters](../02_basic_calculator/02_variables.md#function-arguments-are-variables):
каждая function signature — это contract между caller и callee, условия которого должны быть указаны явно.
Так compiler формирует более понятные сообщения об ошибках, documentation становится лучше, снижается риск непреднамеренных поломок между версиями,
а compilation занимает меньше времени.
