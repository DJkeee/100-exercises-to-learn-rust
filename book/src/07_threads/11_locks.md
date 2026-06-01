# Locks, `Send` и `Arc`

У только что реализованной patching strategy есть серьёзный недостаток: она подвержена race condition.\
Если два clients почти одновременно отправят patches для одной заявки, server применит их в произвольном порядке.
Patch, последним попавший в queue, перезапишет изменения другого client.

## Version numbers

Можно попытаться исправить это с помощью **version number**.\
При создании каждой заявке присваивается version number `0`.\
Отправляя patch, client должен приложить текущий version number заявки к желаемым изменениям.
Server применит patch, только если version number совпадает с сохранённым.

В описанном выше сценарии server отклонит второй patch, поскольку первый patch уже увеличит version number
и он перестанет совпадать с version number, отправленным вторым client.

Этот подход часто используется в distributed systems (например, когда client и servers не используют shared memory)
и называется **optimistic concurrency control**.\
Идея заключается в том, что в большинстве случаев conflicts не будет, поэтому можно оптимизировать распространённый сценарий.
Вы уже достаточно знаете о Rust, чтобы самостоятельно реализовать эту strategy в качестве дополнительного упражнения.

## Locking

Также race condition можно устранить с помощью **lock**.\
Когда client хочет обновить заявку, он должен сначала acquire lock для неё. Пока lock удерживается,
ни один другой client не может изменить заявку.

Standard library Rust предоставляет две locking primitives: `Mutex<T>` и `RwLock<T>`.\
Начнём с `Mutex<T>`. Название образовано от **mut**ual **ex**clusion. Это простейший вид lock:
он разрешает доступ к data только одному thread независимо от того, требуется read или write.

`Mutex<T>` оборачивает защищаемые data, поэтому является generic по их type.\
Напрямую обратиться к data нельзя: type system вынуждает сначала acquire lock с помощью `Mutex::lock` или
`Mutex::try_lock`. Первый method блокируется до acquire lock, а второй немедленно возвращает error, если acquire lock
невозможен.\
Оба methods возвращают guard object, который dereference к data и позволяет изменять их. Lock освобождается,
когда guard dropped.

```rust
use std::sync::Mutex;

// Целое число, защищенное mutex
let lock = Mutex::new(0);

// Захватываем mutex
let mut guard = lock.lock().unwrap();

// Изменяем данные через guard,
// используя его реализацию `Deref`
*guard += 1;

// Блокировка снимается, когда `data` выходит из области видимости
// Это можно сделать явно, удалив guard,
// или неявно, когда guard выходит из области видимости
drop(guard)
```

## Locking granularity

Что должен оборачивать наш `Mutex`?\
Простейший вариант — обернуть весь `TicketStore` в один `Mutex`.\
Это сработает, но серьёзно ограничит performance системы: читать заявки параллельно не получится,
поскольку каждая read operation должна будет ждать освобождения lock.\
Такой подход называется **coarse-grained locking**.

Лучше использовать **fine-grained locking**, при котором каждая заявка защищена собственным lock.
Тогда clients смогут параллельно работать с заявками, пока не обращаются к одной и той же заявке.

```rust
// Новая структура с блокировкой для каждой заявки
struct TicketStore {
    tickets: BTreeMap<TicketId, Mutex<Ticket>>,
}
```

Этот подход эффективнее, но у него есть недостаток: `TicketStore` придётся **учитывать** multithreaded-характер
системы; до сих пор `TicketStore` беззаботно игнорировал существование threads.\
Тем не менее используем этот вариант.

## Кто удерживает lock?

Чтобы схема работала, lock нужно передать client, который хочет изменить заявку.\
После этого client сможет напрямую изменить заявку, как если бы располагал `&mut Ticket`, а затем освободить lock.

Здесь есть сложность.\
Нельзя отправить `Mutex<Ticket>` через channel, поскольку `Mutex` не реализует `Clone`
и его нельзя переместить из `TicketStore`. Можно ли вместо него отправить `MutexGuard`?

Проверим идею на небольшом примере:

```rust
use std::thread::spawn;
use std::sync::Mutex;
use std::sync::mpsc::sync_channel;

fn main() {
    let lock = Mutex::new(0);
    let (sender, receiver) = sync_channel(1);
    let guard = lock.lock().unwrap();

    spawn(move || {
        receiver.recv().unwrap();
    });

    // Пытаемся отправить guard через канал
    // в другой поток
    sender.send(guard);
}
```

Compiler недоволен этим code:

```text
error[E0277]: `MutexGuard<'_, i32>` cannot be sent between 
              threads safely
   --> src/main.rs:10:7
    |
10  |   spawn(move || {
    |  _-----_^
    | | |
    | | required by a bound introduced by this call
11  | |     receiver.recv().unwrap();
12  | | });
    | |_^ `MutexGuard<'_, i32>` cannot be sent between threads safely
    |
    = help: the trait `Send` is not implemented for 
            `MutexGuard<'_, i32>`, which is required by 
            `{closure@src/main.rs:10:7: 10:14}: Send`
    = note: required for `Receiver<MutexGuard<'_, i32>>` 
            to implement `Send`
note: required because it's used within this closure
```

`MutexGuard<'_, i32>` не реализует `Send`: что это означает?

## `Send`

`Send` — marker trait, указывающий, что type можно безопасно передавать из одного thread в другой.\
Кроме того, `Send`, как и `Sized`, является auto trait: compiler автоматически реализует или не реализует его
для вашего type в зависимости от definition.\
Можно реализовать `Send` для своих types вручную, но для этого требуется `unsafe`, поскольку вы должны гарантировать,
что type действительно безопасно передавать между threads по причинам, которые compiler не может проверить автоматически.

### Требования channels

`Sender<T>`, `SyncSender<T>` и `Receiver<T>` реализуют `Send` тогда и только тогда, когда `T` реализует `Send`.\
Причина в том, что они используются для передачи values между threads, а если само value не реализует `Send`,
передавать его между threads было бы unsafe.

### `MutexGuard`

`MutexGuard` не реализует `Send`, поскольку на некоторых платформах primitives operating system,
используемые `Mutex` для реализации lock, требуют освобождать lock в том же thread, который выполнил acquire.\
Если отправить `MutexGuard` в другой thread, lock будет освобождён другим thread, что приведёт к undefined behavior.

## Наши сложности

Подведём итог:

- Нельзя отправить `MutexGuard` через channel. Поэтому нельзя выполнить locking на стороне server, а затем изменить заявку
  на стороне client.
- Можно отправить `Mutex` через channel, поскольку он реализует `Send`, если защищаемые им data реализуют `Send`, как
  в случае с `Ticket`.
  При этом нельзя переместить `Mutex` из `TicketStore` или clone его.

Как решить эту задачу?\
Нужно взглянуть на проблему с другой стороны.
Для locking `Mutex` не требуется owned value. Достаточно shared reference, поскольку `Mutex` использует interior mutability:

```rust
impl<T> Mutex<T> {
    // `&self`, а не `self`!
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        // Детали реализации
    }
}
```

Таким образом, достаточно отправить shared reference client.\
Однако напрямую сделать это нельзя: reference должен быть `'static`, а это не так.\
Нам нужен своего рода «owned shared reference». В Rust есть подходящий type: `Arc`.

## На помощь приходит `Arc`

`Arc` означает **atomic reference counting**.\
`Arc` оборачивает value и отслеживает количество существующих references на него.
Когда последний reference dropped, value deallocated.\
Value внутри `Arc` immutable: получить на него можно только shared references.

```rust
use std::sync::Arc;

let data: Arc<u32> = Arc::new(0);
let data_clone = Arc::clone(&data);

// `Arc<T>` реализует `Deref<T>`, поэтому можно преобразовать
// `&Arc<T>` в `&T` с помощью deref coercion
let data_ref: &u32 = &data;
```

Если это кажется знакомым, вы правы: `Arc` очень похож на `Rc`, reference-counted pointer, с которым мы познакомились
при обсуждении interior mutability. Разница заключается в thread safety: `Rc` не реализует `Send`, а `Arc` реализует.
Всё сводится к реализации reference count: `Rc` использует «обычное» integer, а `Arc` —
**atomic** integer, которое можно безопасно разделять и изменять между threads.

## `Arc<Mutex<T>>`

Объединив `Arc` с `Mutex`, мы наконец получим type, который:

- Можно передавать между threads, поскольку:
  - `Arc` реализует `Send`, если `T` реализует `Send`, и
  - `Mutex` реализует `Send`, если `T` реализует `Send`.
  - `T` — это `Ticket`, реализующий `Send`.
- Можно clone, поскольку `Arc` реализует `Clone` независимо от `T`.
  Cloning `Arc` увеличивает reference count; data не копируются.
- Можно использовать для изменения обёрнутых data, поскольку `Arc` позволяет получить shared
  reference на `Mutex<T>`, который, в свою очередь, позволяет acquire lock.

Теперь у нас есть всё необходимое для реализации стратегии locking в нашем хранилище заявок.

## Дополнительные материалы

- В этом курсе мы не будем подробно рассматривать atomic operations, но дополнительную информацию можно найти
  [в документации `std`](https://doc.rust-lang.org/std/sync/atomic/index.html), а также в книге
  ["Rust atomics and locks"](https://marabos.nl/atomics/).
