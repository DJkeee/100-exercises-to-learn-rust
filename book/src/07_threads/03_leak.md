# Leaking data

The main concern around passing references to spawned threads is use-after-free bugs:
accessing data using a pointer to a memory region that's already been freed/de-allocated.\
If you're working with heap-allocated data, you can avoid the issue by
telling Rust that you'll never reclaim that memory: you choose to **leak memory**,
intentionally.

This can be done, for example, using the `Box::leak` method from Rust's standard library:

```rust
// Выделяем `u32` в куче, оборачивая его в `Box`.
let x = Box::new(41u32);
// С помощью `Box::leak` сообщаем Rust, что никогда не освободим
// выделенную в куче память. Благодаря этому можно получить ссылку 'static.
let static_ref: &'static mut u32 = Box::leak(x);
```

## Data leakage is process-scoped

Leaking data is dangerous: if you keep leaking memory, you'll eventually
run out and crash with an out-of-memory error.

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

At the same time, memory leaked via `leak` method is not truly forgotten.\
The operating system can map each memory region to the process responsible for it.
When the process exits, the operating system will reclaim that memory.

Keeping this in mind, it can be OK to leak memory when:

- The amount of memory you need to leak is bounded/known upfront, or
- Your process is short-lived and you're confident you won't exhaust
  all the available memory before it exits

"Let the OS deal with it" is a perfectly valid memory management strategy
if your usecase allows for it.
