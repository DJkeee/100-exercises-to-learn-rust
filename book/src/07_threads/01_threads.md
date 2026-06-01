# Threads

Прежде чем писать multithreaded code, сделаем шаг назад и поговорим о том, что такое threads
и зачем они могут понадобиться.

## Что такое thread?

**Thread** — это execution context, которым управляет operating system.\
У каждого thread есть собственные stack и instruction pointer.

Один **process** может управлять несколькими threads.
Эти threads используют общее memory space, то есть могут обращаться к одним и тем же data.

Threads — **логическая** конструкция. В конечном счёте на одном CPU core, **физической**
единице выполнения, одновременно может выполняться только один набор инструкций.\
Поскольку threads может быть намного больше, чем CPU cores, **scheduler** operating system
решает, какой thread выполнять в каждый момент времени, распределяя между ними CPU time
для максимальной throughput и responsiveness.

## `main`

При запуске программа Rust выполняется в единственном thread — **main thread**.\
Этот thread создаётся operating system и отвечает за выполнение function `main`.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

## `std::thread`

Standard library Rust предоставляет module `std::thread`, позволяющий создавать
threads и управлять ими.

### `spawn`

С помощью `std::thread::spawn` можно создавать новые threads и выполнять в них code.

Например:

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });
    
    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Hello from the main thread!");
    }
}
```

Если выполнить эту программу в [Rust playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=afedf7062298ca8f5a248bc551062eaa),
вы увидите, что main thread и spawned thread выполняются concurrently.\
Каждый thread продвигается независимо от другого.

### Завершение process

Когда main thread завершается, завершается и весь process.\
Spawned thread продолжает выполняться, пока не завершится сам или пока не завершится main thread.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_secs(1));
            println!("Hello from a thread!");
        }
    });

    thread::sleep(Duration::from_secs(5));
}
```

В примере выше сообщение "Hello from a thread!" будет выведено примерно пять раз.\
Затем main thread завершится (когда вернётся вызов `sleep`), а spawned thread будет остановлен,
поскольку завершится весь process.

### `join`

Также можно дождаться завершения spawned thread, вызвав method `join` у `JoinHandle`, возвращённого `spawn`.

```rust
use std::thread;
fn main() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap();
}
```

В этом примере main thread дождётся завершения spawned thread и только после этого завершится сам.\
Так между двумя threads появляется форма **synchronization**: сообщение
"Hello from a thread!" гарантированно будет выведено до завершения программы, потому что main thread
не завершится, пока не закончит работу spawned thread.
