# Memory leaks

Главная проблема при передаче references в spawned threads — bugs use-after-free:
обращение к data через pointer на уже освобождённый memory region.\
При работе с heap-allocated data проблему можно обойти, сообщив Rust, что эта memory
никогда не будет освобождена: вы намеренно решаете **leak memory**.

Например, это можно сделать с помощью method `Box::leak` из standard library Rust:

```rust
// Выделяем `u32` в куче, оборачивая его в `Box`.
let x = Box::new(41u32);
// С помощью `Box::leak` сообщаем Rust, что никогда не освободим
// выделенную в куче память. Благодаря этому можно получить ссылку 'static.
let static_ref: &'static mut u32 = Box::leak(x);
```

## Memory leak ограничена lifetime process

Memory leak опасна: если продолжать leak memory, в итоге она закончится
и программа аварийно завершится с error out-of-memory.

```rust
// Если оставить этот код работающим на некоторое время,
// в конце концов он использует всю доступную память.
fn oom_trigger() {
    loop {
        let v: Vec<usize> = Vec::with_capacity(1024);
        v.leak();
    }
}
```

При этом memory, leaked с помощью method `leak`, не забыта окончательно.\
Operating system может сопоставить каждый memory region с отвечающим за него process.
При завершении process operating system освободит эту memory.

С учётом этого leak memory может быть допустимо, если:

- Объём memory, которую нужно leak, ограничен или заранее известен либо
- Ваш process недолговечен и вы уверены, что не исчерпаете
  всю доступную memory до его завершения

«Пусть с этим разберётся OS» — вполне допустимая стратегия memory management,
если ваш use case это позволяет.
