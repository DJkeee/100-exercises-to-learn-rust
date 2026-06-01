# Variants могут содержать данные

```rust
enum Status {
    ToDo,
    InProgress,
    Done,
}
```

Наш `Status` enum обычно называют **C-style enum**.\
Каждый variant представляет собой простую метку, похожую на именованную constant. Такой enum встречается во многих
языках программирования: C, C++, Java, C#, Python и других.

Но enums в Rust способны на большее. К каждому variant можно **прикрепить данные**.

## Variants

Предположим, мы хотим хранить имя человека, который сейчас работает над ticket.\
Эта информация будет доступна только для ticket в процессе выполнения. У ticket со status to-do или done
её не будет.
Это можно смоделировать, прикрепив field `String` к variant `InProgress`:

```rust
enum Status {
    ToDo,
    InProgress {
        assigned_to: String,
    },
    Done,
}
```

Теперь `InProgress` — это **struct-like variant**.\
Syntax фактически повторяет syntax объявления struct: оно просто «встроено» внутрь enum в качестве variant.

## Доступ к данным variant

Если попытаться обратиться к `assigned_to` у instance `Status`,

```rust
let status: Status = /* */;

// Это не скомпилируется
println!("Assigned to: {}", status.assigned_to);
```

compiler не позволит это сделать:

```text
error[E0609]: no field `assigned_to` on type `Status`
 --> src/main.rs:5:40
  |
5 |     println!("Assigned to: {}", status.assigned_to);
  |                                        ^^^^^^^^^^^ unknown field
```

`assigned_to` является **variant-specific**: оно доступно не для всех instances `Status`.\
Чтобы обратиться к `assigned_to`, необходимо использовать **pattern matching**:

```rust
match status {
    Status::InProgress { assigned_to } => {
        println!("Assigned to: {}", assigned_to);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```

## Bindings

В match pattern `Status::InProgress { assigned_to }` элемент `assigned_to` является **binding**.\
Мы выполняем **destructuring** variant `Status::InProgress` и привязываем field `assigned_to`
к новой variable, которая также называется `assigned_to`.\
При желании field можно привязать к variable с другим именем:

```rust
match status {
    Status::InProgress { assigned_to: person } => {
        println!("Assigned to: {}", person);
    },
    Status::ToDo | Status::Done => {
        println!("ToDo or Done");
    }
}
```
