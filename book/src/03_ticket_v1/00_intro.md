# Моделирование ticket

Первая глава должна была дать вам хорошее представление о некоторых primitive types Rust, операторах и
базовых конструкциях control flow.\
В этой главе мы сделаем ещё один шаг и разберём то, что делает Rust по-настоящему уникальным: **ownership**.\
Именно ownership позволяет Rust обеспечивать memory safety и высокую производительность без garbage collector.

В качестве сквозного примера мы возьмём ticket наподобие JIRA: такие ticket используют для отслеживания bugs, features или tasks в
software project.\
Мы попробуем смоделировать его на Rust. Это будет первая итерация: к концу главы она не станет ни идеальной, ни вполне idiomatic,
но задача всё равно будет достаточно сложной!\
Чтобы двигаться дальше, вам придётся освоить несколько новых концепций Rust:

- `struct` — один из способов определять custom types в Rust
- ownership, references и borrowing
- memory management: stack, heap, pointers, data layout, destructors
- modules и visibility
- strings
