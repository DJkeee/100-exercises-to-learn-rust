# Dependencies

Package может зависеть от других packages, перечисленных в section `[dependencies]` его файла `Cargo.toml`.\
Чаще всего dependency указывают с помощью имени и version:

```toml
[dependencies]
thiserror = "1"
```

Так `thiserror` будет добавлен в package как dependency с **минимальной** version `1.0.0`.
`thiserror` будет загружен из [crates.io](https://crates.io), официального package registry Rust.
При запуске `cargo build` инструмент `cargo` выполняет несколько этапов:

- Dependency resolution
- Загрузка dependencies
- Compilation проекта: собственного codeа и dependencies

Dependency resolution пропускается, если в проекте есть файл `Cargo.lock`, а файлы manifest не изменились.
После успешного dependency resolution инструмент `cargo` автоматически создаёт lockfile: он содержит
точные versions всех используемых в проекте dependencies и гарантирует, что в разных builds,
например в CI, стабильно используются одни и те же versions. Если над проектом работают несколько developers,
файл `Cargo.lock` следует commit в version control system.

Команда `cargo update` позволяет обновить файл `Cargo.lock` до последних совместимых versions всех dependencies.

### Path dependencies

Dependency также можно указать с помощью **path**. Это удобно при работе с несколькими локальными packages.

```toml
[dependencies]
my-library = { path = "../my-library" }
```

Path указывается относительно файла `Cargo.toml` package, объявляющего dependency.

### Другие sources

Подробнее о sources dependencies и способах их указания в файле `Cargo.toml` см.
в [documentation Cargo](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html).

## Dev dependencies

Можно также указать dependencies, необходимые только для development: они загружаются только при
запуске `cargo test`.\
Их помещают в section `[dev-dependencies]` файла `Cargo.toml`:

```toml
[dev-dependencies]
static_assertions = "1.1.0"
```

На протяжении книги мы уже использовали несколько таких dependencies, чтобы сократить tests.
