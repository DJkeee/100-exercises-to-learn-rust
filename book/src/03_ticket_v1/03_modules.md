# Modules

Только что определённый вами method `new` пытается обеспечить соблюдение некоторых **ограничений** для значений fields `Ticket`.
Но действительно ли эти invariants соблюдаются? Что мешает разработчику создать `Ticket`,
не используя `Ticket::new`?

Чтобы добиться полноценной **encapsulation**, нужно познакомиться с двумя новыми концепциями: **visibility** и **modules**.
Начнём с modules.

## Что такое module?

В Rust **module** позволяет объединить связанный код в общем namespace, то есть под именем module.\
Вы уже видели modules в действии: unit tests, проверяющие корректность вашего кода, определены в
отдельном module с именем `tests`.

```rust
#[cfg(test)]
mod tests {
    // [...]
}
```

## Inline modules

Module `tests` выше — это пример **inline module**: объявление module (`mod tests`) и его
содержимое (всё, что находится внутри `{ ... }`) расположены рядом.

## Module tree

Modules могут быть вложенными, образуя **tree**.\
Корень этого tree — сам **crate**, то есть module верхнего уровня, содержащий все остальные modules.
Для library crate корневым module обычно служит `src/lib.rs` (если его расположение не было изменено).
Корневой module также называют **crate root**.

У crate root могут быть submodules, у которых, в свою очередь, могут быть собственные submodules, и так далее.

## External modules и filesystem

Inline modules удобны для небольших фрагментов кода, но по мере роста проекта вам захочется разделить код между
несколькими файлами. В parent module существование submodule объявляется с помощью keyword `mod`.

```rust
mod dog;
```

После этого `cargo`, build tool Rust, должен найти файл, содержащий
реализацию module.\
Если ваш module объявлен в корне crate (например, в `src/lib.rs` или `src/main.rs`),
`cargo` ожидает, что файл будет называться одним из двух способов:

- `src/<module_name>.rs`
- `src/<module_name>/mod.rs`

Если ваш module является submodule другого module, файл должен называться так:

- `[..]/<parent_module>/<module_name>.rs`
- `[..]/<parent_module>/<module_name>/mod.rs`

Например, `src/animals/dog.rs` или `src/animals/dog/mod.rs`, если `dog` — submodule `animals`.

Ваша IDE может помочь автоматически создать эти файлы при объявлении нового module с помощью keyword `mod`.

## Item paths и statements `use`

К items, определённым в том же module, можно обращаться без специального syntax: достаточно использовать их имя.

```rust
struct Ticket {
    // [...]
}

// Здесь не нужно уточнять `Ticket`,
// поскольку мы находимся в том же модуле
fn mark_ticket_as_done(ticket: Ticket) {
    // [...]
}
```

Если же вы хотите обратиться к сущности из другого module, этого недостаточно.\
Необходимо использовать **path**, указывающий на нужную сущность.

Path можно составить несколькими способами:

- начиная от корня текущего crate, например `crate::module_1::MyStruct`
- начиная от parent module, например `super::my_function`
- начиная от текущего module, например `sub_module_1::MyStruct`

И `crate`, и `super` — **keywords**.\
`crate` указывает на корень текущего crate, а `super` — на parent текущего module.

Писать полный path при каждом обращении к type может быть неудобно.
Чтобы упростить задачу, можно добавить statement `use`, который вводит сущность в scope.

```rust
// Вводим `MyStruct` в область видимости
use crate::module_1::module_2::MyStruct;

// Теперь можно обращаться к `MyStruct` напрямую
fn a_function(s: MyStruct) {
     // [...]
}
```

### Star imports

Также можно импортировать все items из module одним statement `use`.

```rust
use crate::module_1::module_2::*;
```

Это называется **star import**.\
Обычно использовать его не рекомендуют: он может засорить текущий namespace, из-за чего становится трудно понять,
откуда взялось каждое имя, а также может привести к конфликтам имён.\
Тем не менее в некоторых случаях он бывает полезен, например при написании unit tests. Возможно, вы заметили,
что большинство наших test modules начинаются со statement `use super::*;`, который вводит в scope все items из parent module
(того, который тестируется).

## Визуализация module tree

Если вам трудно представить module tree своего проекта, попробуйте визуализировать его с помощью
[`cargo-modules`](https://crates.io/crates/cargo-modules)!

Инструкции по установке и примеры использования приведены в документации.
