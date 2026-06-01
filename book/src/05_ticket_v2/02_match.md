# `match`

Возможно, вы задаётесь вопросом: что вообще можно **делать** с enum?\
Наиболее распространённая operation — применить к нему **match**.

```rust
enum Status {
    ToDo,
    InProgress,
    Done
}

impl Status {
    fn is_done(&self) -> bool {
        match self {
            Status::Done => true,
            // Оператор `|` позволяет сопоставлять несколько паттернов.
            // Читается как «либо `Status::ToDo`, либо `Status::InProgress`».
            Status::InProgress | Status::ToDo => false
        }
    }
}
```

Statement `match` позволяет match value Rust с последовательностью **patterns**.\
Его можно представить как `if` на уровне types. Если `status` — это variant `Done`, выполняется первый block;
если variant `InProgress` или `ToDo` — второй.

## Exhaustiveness

Здесь есть одна важная деталь: `match` является **exhaustive**. Необходимо обработать все enum variants.\
Если забыть обработать какой-либо variant, Rust выдаст error **at compile-time**.

Например, если забыть обработать variant `ToDo`:

```rust
match self {
    Status::Done => true,
    Status::InProgress => false,
}
```

compiler сообщит об error:

```text
error[E0004]: non-exhaustive patterns: `ToDo` not covered
 --> src/main.rs:5:9
  |
5 |     match status {
  |     ^^^^^^^^^^^^ pattern `ToDo` not covered
```

Это очень важно!\
Codebase со временем развивается: в дальнейшем вы можете добавить новый status, например `Blocked`. Compiler Rust
выдаст error для каждого statement `match`, в котором отсутствует logic для нового variant.
Именно поэтому Rust developers часто хвалят «compiler-driven refactoring»: compiler подсказывает,
что делать дальше, а вам остаётся исправить то, о чём он сообщает.

## Catch-all

Если один или несколько variants вас не интересуют, можно использовать pattern `_` в качестве catch-all:

```rust
match status {
    Status::Done => true,
    _ => false
}
```

Pattern `_` соответствует всему, что не подошло под предыдущие patterns.

<div class="warning">
При использовании этого catch-all pattern вы _не получите_ преимуществ compiler-driven refactoring.
Если добавить новый enum variant, compiler _не сообщит_, что вы его не обрабатываете.

Если для вас важна корректность, избегайте catch-all patterns. Используйте compiler, чтобы заново check все места matching и определить, как следует обрабатывать новые enum variants.

</div>
