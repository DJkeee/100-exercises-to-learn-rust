# Копирование values, часть 1

В предыдущей главе мы познакомились с ownership и borrowing.\
В частности, мы сформулировали следующие правила:

- У каждого value в Rust в любой момент времени есть ровно один owner.
- Когда function забирает ownership value («consumes» его), caller больше не может использовать это value.

Эти ограничения иногда могут мешать.\
Порой необходимо вызвать function, забирающую ownership value, но после этого value всё ещё
нужно использовать.

```rust
fn consumer(s: String) { /* */ }

fn example() {
     let mut s = String::from("hello");
     consumer(s);
     s.push_str(", world!"); // ошибка: значение заимствовано здесь после перемещения
}
```

Здесь на помощь приходит `Clone`.

## `Clone`

`Clone` — trait, определённый в standard library Rust:

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

Его method `clone` принимает reference на `self` и возвращает новый **owned** instance того же type.

## На практике

Вернувшись к примеру выше, можно использовать `clone`, чтобы создать новый instance `String` перед вызовом `consumer`:

```rust
fn consumer(s: String) { /* */ }

fn example() {
     let mut s = String::from("hello");
     let t = s.clone();
     consumer(t);
     s.push_str(", world!"); // ошибки нет
}
```

Вместо передачи ownership `s` в `consumer` мы создаём новый `String` с помощью cloning `s` и передаём
в `consumer` именно его.\
После вызова `consumer` value `s` остаётся действительным и доступным для использования.

## В memory

Рассмотрим, что произошло в memory в примере выше.
После выполнения `let mut s = String::from("hello");` memory выглядит так:

```text
                    s
      +---------+--------+----------+
Stack | pointer | length | capacity | 
      |  |      |   5    |    5     |
      +--|------+--------+----------+
         |
         |
         v
       +---+---+---+---+---+
Heap:  | H | e | l | l | o |
       +---+---+---+---+---+
```

После выполнения `let t = s.clone()` в heap выделяется совершенно новая область для хранения копии данных:

```text
                    s                                    t
      +---------+--------+----------+      +---------+--------+----------+
Stack | pointer | length | capacity |      | pointer | length | capacity |
      |  |      |   5    |    5     |      |  |      |   5    |    5     |
      +--|------+--------+----------+      +--|------+--------+----------+
         |                                    |
         |                                    |
         v                                    v
       +---+---+---+---+---+                +---+---+---+---+---+
Heap:  | H | e | l | l | o |                | H | e | l | l | o |
       +---+---+---+---+---+                +---+---+---+---+---+
```

Если вы знакомы с языком вроде Java, можно считать `clone` способом создать deep copy объекта.

## Implementation `Clone`

Чтобы type можно было cloning, для него необходимо реализовать trait `Clone`.\
Почти всегда `Clone` реализуют с помощью derive:

```rust
#[derive(Clone)]
struct MyType {
    // поля
}
```

Compiler реализует `Clone` для `MyType` ожидаемым образом: отдельно выполняет cloning каждого field `MyType`,
а затем создаёт новый instance `MyType` из cloned fields.\
Помните, что для изучения кода, сгенерированного derive macros, можно использовать `cargo expand` или IDE.
