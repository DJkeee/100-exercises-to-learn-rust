# `'static`

Если в предыдущем упражнении вы пытались borrow slice из vector,
то, скорее всего, получили примерно такую compiler error:

```text
error[E0597]: `v` does not live long enough
   |
11 | pub fn sum(v: Vec<i32>) -> i32 {
   |            - binding `v` declared here
...
15 |     let right = &v[split_point..];
   |                  ^ borrowed value does not live long enough
16 |     let left_handle = spawn(move || left.iter().sum::<i32>());
   |                             -------------------------------- 
                     argument requires that `v` is borrowed for `'static`
19 | }
   |  - `v` dropped here while still borrowed
```

Что означает `argument requires that v is borrowed for 'static`?

`'static` lifetime — особый lifetime в Rust.\
Он означает, что value будет действительно на протяжении всего runtime программы.

## Detached threads

Thread, запущенный через `thread::spawn`, может **outlive** породивший его thread.\
Например:

```rust
use std::thread;

fn f() {
    thread::spawn(|| {
        thread::spawn(|| {
            loop {
                thread::sleep(std::time::Duration::from_secs(1));
                println!("Hello from the detached thread!");
            }
        });
    });
}
```

В этом примере первый spawned thread, в свою очередь, породит
child thread, который каждую секунду выводит сообщение.\
Затем первый thread завершится. После этого его child thread будет
**продолжать выполняться**, пока работает весь process.\
В терминологии Rust говорят, что child thread **outlived**
свой parent thread.

## `'static` lifetime

Поскольку spawned thread может:

- outlive породивший его thread (его parent thread)
- выполняться до завершения программы

он не должен borrow values, которые могут быть dropped до завершения программы;
нарушение этого ограничения привело бы к bug use-after-free.\
Именно поэтому signature `std::thread::spawn` требует, чтобы переданная ему closure
имела lifetime `'static`:

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T> 
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
{
    // [..]
}
```

## `'static` относится не только к references

В Rust lifetime есть у всех values, а не только у references.

В частности, type, владеющий своими data (например, `Vec` или `String`),
удовлетворяет ограничению `'static`: если value принадлежит вам, с ним можно работать
сколь угодно долго, даже после возврата из function, которая изначально его создала.

Таким образом, `'static` можно понимать как требование:

- Дайте мне owned value
- Дайте мне reference, действительную на протяжении всего runtime программы

Первый подход вы использовали для решения задачи в предыдущем упражнении:
выделили новые vectors для левой и правой частей исходного vector,
после чего переместили их в spawned threads.

## References с `'static`

Поговорим о втором случае: references, действительных на протяжении всего
runtime программы.

### Static data

Наиболее распространённый случай — reference на **static data**, например string literals:

```rust
let s: &'static str = "Hello world!";
```

Поскольку string literals известны в compile time, Rust хранит их _внутри_ executable
в области, называемой **read-only data segment**.
Поэтому все references, указывающие на эту область, действительны всё время работы
программы и удовлетворяют контракту `'static`.

## Дополнительные материалы

- [Data segment](https://en.wikipedia.org/wiki/Data_segment)
