# Readers и writers

Новый `TicketStore` работает, но его performance при read operations оставляет желать лучшего: конкретную заявку
может одновременно читать только один client, поскольку `Mutex<T>` не различает readers и writers.

Проблему можно решить с помощью другой locking primitive: `RwLock<T>`.\
`RwLock<T>` означает **read-write lock**. Он разрешает одновременный доступ к data **нескольким readers**,
но только одному writer.

У `RwLock<T>` есть два methods для acquire lock: `read` и `write`.\
`read` возвращает guard для read access к data, а `write` — guard для их изменения.

```rust
use std::sync::RwLock;

// Целое число, защищенное read-write lock
let lock = RwLock::new(0);

// Захватываем блокировку чтения RwLock
let guard1 = lock.read().unwrap();

// Захватываем **вторую** блокировку чтения,
// пока первая еще активна
let guard2 = lock.read().unwrap();
```

## Trade-offs

На первый взгляд выбор `RwLock<T>` очевиден: он предоставляет superset возможностей `Mutex<T>`.
Зачем вообще использовать `Mutex<T>`, если вместо него можно взять `RwLock<T>`?

Есть две основные причины:

- Locking `RwLock<T>` дороже, чем locking `Mutex<T>`.\
  Причина в том, что `RwLock<T>` должен отслеживать количество активных readers и writers, а `Mutex<T>` —
  только факт удержания lock.
  Эти дополнительные расходы несущественны, если readers больше, чем writers, но при write-heavy workload
  `Mutex<T>` может оказаться лучшим выбором.
- `RwLock<T>` может вызвать **writer starvation**.\
  Если readers постоянно ожидают acquire lock, writers могут вообще не получить возможности выполниться.\
  `RwLock<T>` не гарантирует порядок предоставления доступа к lock для readers и writers.
  Он зависит от policy базовой OS, которая может быть несправедливой к writers.

В нашем случае workload, скорее всего, будет read-heavy, поскольку большинство clients будут читать заявки,
а не изменять их. Поэтому `RwLock<T>` — подходящий выбор.
