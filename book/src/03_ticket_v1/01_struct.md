# Structs

Для каждого ticket нам нужно хранить три элемента данных:

- title
- description
- status

Для начала представим их с помощью [`String`](https://doc.rust-lang.org/std/string/struct.String.html).
`String` — это type из standard library Rust, предназначенный для представления текста в
[кодировке UTF-8](https://en.wikipedia.org/wiki/UTF-8).

Но как **объединить** эти три элемента данных в единую сущность?

## Определение `struct`

`struct` определяет **новый type Rust**.

```rust
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

Конструкция struct во многом похожа на то, что в других языках программирования называют class или object.

## Определение fields

Новый type создаётся объединением других types в **fields**.\
У каждого field должны быть имя и type, разделённые двоеточием `:`. Если fields несколько, они разделяются запятыми `,`.

Fields не обязаны иметь одинаковый type, как видно из struct `Configuration` ниже:

```rust
struct Configuration {
   version: u32,
   active: bool
}
```

## Создание instance

Можно создать instance struct, указав значения каждого field:

```rust
// Синтаксис: <StructName> { <field_name>: <value>, ... }
let ticket = Ticket {
    title: "Build a ticket system".into(),
    description: "A Kanban board".into(),
    status: "Open".into()
};
```

## Доступ к fields

Получить доступ к fields struct можно с помощью оператора `.`:

```rust
// Доступ к полю
let x = ticket.description;
```

## Methods

Мы можем добавить поведение structs, определив **methods**.\
Рассмотрим это на примере struct `Ticket`:

```rust
impl Ticket {
    fn is_open(self) -> bool {
        self.status == "Open"
    }
}

// Синтаксис:
// impl <StructName> {
//    fn <method_name>(<parameters>) -> <return_type> {
//        // Тело метода
//    }
// }
```

Methods очень похожи на functions, но есть два ключевых отличия:

1. methods должны быть определены внутри **блока `impl`**
2. methods могут принимать `self` в качестве первого parameter.
   `self` — это keyword, обозначающий instance struct, для которого вызывается method.

### `self`

Если method принимает `self` в качестве первого parameter, его можно вызвать с помощью **method call syntax**:

```rust
// Синтаксис вызова метода: <instance>.<method_name>(<parameters>)
let is_open = ticket.is_open();
```

Именно этот syntax вызова вы использовали для выполнения saturating arithmetic операций со значениями `u32`
в [предыдущей главе](../02_basic_calculator/09_saturating.md).

### Static methods

Если method не принимает `self` в качестве первого parameter, это **static method**.

```rust
struct Configuration {
    version: u32,
    active: bool
}

impl Configuration {
    // `default` — статический метод `Configuration`
    fn default() -> Configuration {
        Configuration { version: 0, active: false }
    }
}
```

Static method можно вызвать только с помощью **function call syntax**:

```rust
// Синтаксис вызова функции: <StructName>::<method_name>(<parameters>)
let default_config = Configuration::default();
```

### Эквивалентность

Function call syntax можно использовать даже для methods, которые принимают `self` в качестве первого parameter:

```rust
// Синтаксис вызова функции:
//   <StructName>::<method_name>(<instance>, <parameters>)
let is_open = Ticket::is_open(ticket);
```

Function call syntax ясно показывает, что `ticket` используется как `self`, то есть первый parameter method,
но такая запись заметно многословнее. По возможности предпочитайте method call syntax.
