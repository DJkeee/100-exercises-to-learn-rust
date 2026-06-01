# Unwrapping

`Ticket::new` now returns a `Result` instead of panicking on invalid inputs.\
What does this mean for the caller?

## Failures can't be (implicitly) ignored

Unlike exceptions, Rust's `Result` forces you to **handle errors at the call site**.\
If you call a function that returns a `Result`, Rust won't allow you to implicitly ignore the error case.

```rust
fn parse_int(s: &str) -> Result<i32, ParseIntError> {
    // ...
}

// Это не скомпилируется: мы не обрабатываем случай ошибки.
// Нужно использовать `match` или один из комбинаторов `Result`,
// чтобы «развернуть» успешное значение или обработать ошибку.
let number = parse_int("42") + 2;
```

## You got a `Result`. Now what?

When you call a function that returns a `Result`, you have two key options:

- Panic if the operation failed.
  This is done using either the `unwrap` or `expect` methods.
  ```rust
  // Вызывает панику, если `parse_int` возвращает `Err`.
  let number = parse_int("42").unwrap();
  // `expect` позволяет указать собственное сообщение паники.
  let number = parse_int("42").expect("Failed to parse integer");
  ```
- Destructure the `Result` using a `match` expression to deal with the error case explicitly.
  ```rust
  match parse_int("42") {
      Ok(number) => println!("Parsed number: {}", number),
      Err(err) => eprintln!("Error: {}", err),
  }
  ```
