# `.iter()`

`IntoIterator` **consumes** `self`, чтобы создать iterator.

У этого есть преимущество: iterator выдаёт **owned** values.
Например, если вызвать `.into_iter()` для `Vec<Ticket>`, получится iterator, возвращающий значения `Ticket`.

Но в этом же заключается и недостаток: после вызова `.into_iter()` исходную collection больше нельзя использовать.
Зачастую требуется выполнить iteration по collection, не consuming её, а просматривая **references** на значения.
В случае `Vec<Ticket>` потребуется iteration по значениям `&Ticket`.

Большинство collections предоставляют метод `.iter()`, который возвращает iterator по references на элементы collection.
Например:

```rust
let numbers: Vec<u32> = vec![1, 2];
// Здесь `n` имеет тип `&u32`
for n in numbers.iter() {
    // [...]
}
```

Этот pattern можно упростить, реализовав `IntoIterator` для **reference на collection**.
В приведённом выше примере это будет `&Vec<Ticket>`.\
Именно так поступает standard library, поэтому следующий код работает:

```rust
let numbers: Vec<u32> = vec![1, 2];
// Здесь `n` имеет тип `&u32`
// Нам не пришлось явно вызывать `.iter()`
// Достаточно было использовать `&numbers` в цикле `for`
for n in &numbers {
    // [...]
}
```

Idiomatic подход — предоставить оба варианта:

- Implementation `IntoIterator` для reference на collection.
- Метод `.iter()`, возвращающий iterator по references на элементы collection.

Первый вариант удобен в циклах `for`, второй более явный и может использоваться в других контекстах.
