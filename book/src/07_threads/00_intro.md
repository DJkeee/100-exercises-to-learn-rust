# Введение

Одно из главных обещаний Rust — _fearless concurrency_: возможность проще писать безопасные concurrent-программы.
Пока мы почти не видели этого в действии. Вся проделанная до сих пор работа была single-threaded.
Пора это изменить!

В этой главе мы сделаем наше хранилище заявок multithreaded.\
У нас будет возможность познакомиться с большинством основных возможностей Rust для concurrency, включая:

- Threads с использованием module `std::thread`
- Message passing с использованием channels
- Shared state с использованием `Arc`, `Mutex` и `RwLock`
- `Send` и `Sync` — traits, выражающие гарантии Rust в отношении concurrency

Мы также обсудим различные design patterns для multithreaded-систем и некоторые связанные с ними trade-offs.
