# Error enums

Решение предыдущего упражнения могло показаться неудобным: сопоставление strings далеко от идеала!\
Коллега может переработать messages об errors, возвращаемые `Ticket::new`, например для улучшения читаемости,
и вызывающий code внезапно перестанет работать.

Вы уже знакомы с механизмом, позволяющим это исправить: enums!

## Реакция на errors

Если вызывающая сторона должна вести себя по-разному в зависимости от конкретного возникшего error,
различные случаи error можно представить с помощью enum:

```rust
// Перечисление ошибок для представления различных ошибок,
// которые могут возникнуть при парсинге `u32` из строки.
enum U32ParseError {
    NotANumber,
    TooLarge,
    Negative,
}
```

Используя error enum, вы codeируете различные случаи error в type system: они становятся частью
signature fallible function.\
Это упрощает error handling для вызывающей стороны: с помощью expression `match` она может по-разному реагировать
на различные случаи error:

```rust
match s.parse_u32() {
    Ok(n) => n,
    Err(U32ParseError::Negative) => 0,
    Err(U32ParseError::TooLarge) => u32::MAX,
    Err(U32ParseError::NotANumber) => {
        panic!("Not a number: {}", s);
    }
}
```
