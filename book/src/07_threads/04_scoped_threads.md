# Scoped threads

У всех рассмотренных до сих пор проблем с lifetime общий источник:
spawned thread может outlive свой parent thread.\
Эту проблему можно обойти с помощью **scoped threads**.

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

std::thread::scope(|scope| {
    scope.spawn(|| {
        let first = &v[..midpoint];
        println!("Here's the first half of v: {first:?}");
    });
    scope.spawn(|| {
        let second = &v[midpoint..];
        println!("Here's the second half of v: {second:?}");
    });
});

println!("Here's v: {v:?}");
```

Разберём происходящее.

## `scope`

Function `std::thread::scope` создаёт новый **scope**.\
`std::thread::scope` принимает closure с единственным argument — instance `Scope`.

## Scoped spawns

`Scope` предоставляет method `spawn`.\
В отличие от `std::thread::spawn`, все threads, spawned с помощью `Scope`,
будут **автоматически joined** при завершении scope.

Если «переписать» предыдущий пример с использованием `std::thread::spawn`,
он будет выглядеть так:

```rust
let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

let handle1 = std::thread::spawn(|| {
    let first = &v[..midpoint];
    println!("Here's the first half of v: {first:?}");
});
let handle2 = std::thread::spawn(|| {
    let second = &v[midpoint..];
    println!("Here's the second half of v: {second:?}");
});

handle1.join().unwrap();
handle2.join().unwrap();

println!("Here's v: {v:?}");
```

## Borrowing из окружения

Однако переписанный пример не будет compile: compiler сообщит,
что `&v` нельзя использовать из наших spawned threads, поскольку его lifetime
не является `'static`.

С `std::thread::scope` такой проблемы нет: можно **безопасно borrow из окружения**.

В нашем примере `v` создаётся до spawning points.
Он будет dropped только _после_ возврата из `scope`. При этом все threads,
spawned внутри `scope`, гарантированно завершатся _до_ возврата из `scope`,
поэтому риск появления dangling references отсутствует.

Compiler не будет возражать!
