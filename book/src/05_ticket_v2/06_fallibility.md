# Fallibility

Вернёмся к function `Ticket::new` из предыдущего упражнения:

```rust
impl Ticket {
    pub fn new(
        title: String, 
        description: String, 
        status: Status
    ) -> Ticket {
        if title.is_empty() {
            panic!("Title cannot be empty");
        }
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes");
        }
        if description.is_empty() {
            panic!("Description cannot be empty");
        }
        if description.len() > 500 {
            panic!("Description cannot be longer than 500 bytes");
        }

        Ticket {
            title,
            description,
            status,
        }
    }
}
```

Как только одна из checks завершается неудачей, function вызывает `panic`.
Это неидеально, поскольку вызывающая сторона не получает возможности **обработать error**.

Пришло время познакомиться с type `Result`, основным механизмом error handling в Rust.

## Type `Result`

Type `Result` — это enum, defined в standard library:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

У него есть два variants:

- `Ok(T)` представляет успешную operation. Он содержит `T` — результат operation.
- `Err(E)` представляет неудачную operation. Он содержит `E` — возникший error.

И `Ok`, и `Err` являются generic, поэтому для случаев успеха и error можно указывать собственные types.

## Без exceptions

Recoverable errors в Rust **представляются values**.\
Это обычные instances type: они передаются и обрабатываются так же, как любые другие value.
В этом состоит значительное отличие от других языков, например Python или C#, где для messages об errors используются **exceptions**.

Exceptions создают отдельный control flow path, который бывает сложно анализировать.\
По одной лишь signature function нельзя определить, может ли она выбросить exception.
По одной лишь signature function также нельзя определить, **какие именно** types exceptions она может выбросить.\
Чтобы выяснить это, необходимо прочитать documentation function или изучить её implementation.

Logic обработки exceptions обладает очень плохой locality: code, выбрасывающий exception, находится далеко от codeа,
который его перехватывает, и между ними нет прямой связи.

## Fallibility codeируется в type system

Благодаря `Result` Rust заставляет **codeировать fallibility в signature function**.\
Если function может завершиться неудачей и вызывающей стороне необходимо дать возможность обработать error, она должна возвращать `Result`.

```rust
// Уже по сигнатуре видно, что эта функция может завершиться
// ошибкой. Также можно изучить `ParseIntError`, чтобы понять,
// какие ошибки возможны.
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}
```

В этом и состоит главное преимущество `Result`: fallibility становится явной.

Однако следует помнить о существовании panics. Они не отслеживаются type system, как и exceptions в других языках.
Но panics предназначены для **unrecoverable errors**, и использовать их следует редко.
