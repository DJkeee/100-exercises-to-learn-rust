# Libraries и binaries

Чтобы implement trait `Error` для `TicketNewError`, потребовалось немало codeа, не так ли?\
Manual implementation `Display` плюс `impl` block для `Error`.

Часть boilerplate можно устранить с помощью [`thiserror`](https://docs.rs/thiserror/latest/thiserror/):
этот crate Rust предоставляет **procedural macro**, упрощающий создание custom error types.\
Но мы забегаем вперёд: `thiserror` — third-party crate, и он станет нашей первой dependency!

Прежде чем углубляться в dependencies, сделаем шаг назад и обсудим packaging system Rust.

## Что такое package?

Package Rust defined в section `[package]` файла `Cargo.toml`, также известного как **manifest**.
В `[package]` можно указать metadata package, например его имя и version.

Посмотрите файл `Cargo.toml` в директории упражнения для этого раздела!

## Что такое crate?

В package может находиться один или несколько **crates**, также известных как **targets**.\
Два наиболее распространённых crate types — **binary crates** и **library crates**.

### Binaries

Binary — это программа, которую можно compile в **executable file**.\
Она должна содержать function `main` — entry point программы. При запуске программы вызывается `main`.

### Libraries

Libraries, напротив, не являются executable сами по себе. Library нельзя _запустить_,
но её code можно _импортировать_ из другого package, который от неё зависит.\
Library объединяет code, например function и types, который другие packages могут использовать как **dependency**.

Все выполненные до сих пор упражнения были организованы как libraries с прикреплённым test suite.

### Conventions

Следует помнить о нескольких conventions, связанных с packages Rust:

- Source code package обычно находится в директории `src`.
- При наличии файла `src/lib.rs` инструмент `cargo` определит, что package содержит library crate.
- При наличии файла `src/main.rs` инструмент `cargo` определит, что package содержит binary crate.

Эти defaults можно переопределить, явно defining targets в файле `Cargo.toml`. Подробнее см.
в [documentation `cargo`](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#cargo-targets).

Помните: хотя package может содержать несколько crates, library crate в нём может быть только один.
