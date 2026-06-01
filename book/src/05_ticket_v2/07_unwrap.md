# Unwrapping

Теперь `Ticket::new` возвращает `Result`, а не вызывает `panic` при недопустимых входных данных.\
Что это означает для вызывающей стороны?

## Неудачи нельзя игнорировать неявно

В отличие от exceptions, `Result` в Rust заставляет **обрабатывать errors в call site**.\
Если вызвать function, возвращающую `Result`, Rust не позволит неявно проигнорировать случай error.

```rust
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}

// Это не скомпилируется: мы не обрабатываем случай ошибки.
// Нужно использовать `match` или один из комбинаторов `Result`,
// чтобы «развернуть» успешное значение или обработать ошибку.
let number = parse_int("42") + 2;
```

## Получен `Result`. Что дальше?

При вызове function, возвращающей `Result`, есть два основных варианта:

- Вызвать `panic`, если operation завершилась неудачей.
  Для этого используются methods `unwrap` или `expect`.
  ```rust
  // Вызывает панику, если `parse_int` возвращает `Err`.
  let number = parse_int("42").unwrap();
  // `expect` позволяет указать собственное сообщение паники.
  let number = parse_int("42").expect("Failed to parse integer");
  ```
- Выполнить destructuring `Result` с помощью expression `match`, чтобы явно обработать случай error.
  ```rust
  match parse_int("42") {
      Ok(number) => println!("Parsed number: {}", number),
      Err(err) => eprintln!("Error: {}", err),
  }
  ```
