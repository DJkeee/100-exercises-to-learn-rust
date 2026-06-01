# `Sync`

Прежде чем завершить главу, поговорим ещё об одном ключевом trait в standard library Rust: `Sync`.

`Sync`, как и `Send`, является auto trait.\
Он автоматически реализуется для всех types, которые можно безопасно **share** между threads.

Иными словами: `T` реализует `Sync`, если `&T` реализует `Send`.

## `T: Sync` doesn't imply `T: Send`

Важно отметить, что `T` может реализовывать `Sync`, не реализуя `Send`.\
Например, `MutexGuard` не реализует `Send`, но реализует `Sync`.

Он не реализует `Send`, поскольку lock должен быть освобождён в том же thread, который его acquire, а значит,
мы не хотим, чтобы `MutexGuard` был dropped в другом thread.\
Но он реализует `Sync`, поскольку передача `&MutexGuard` другому thread не влияет на место освобождения lock.

## `T: Send` doesn't imply `T: Sync`

Верно и обратное: `T` может реализовывать `Send`, не реализуя `Sync`.\
Например, `RefCell<T>` реализует `Send`, если `T` реализует `Send`, но не реализует `Sync`.

`RefCell<T>` выполняет runtime borrow checking, но counters для отслеживания borrows не являются thread-safe.
Поэтому наличие `&RefCell` у нескольких threads привело бы к data race: несколько threads могли бы получить
mutable references на одни и те же data. Вот почему `RefCell` не реализует `Sync`.\
А `Send` допустим: отправляя `RefCell` другому thread, мы не оставляем references на содержащиеся в нём data,
поэтому риска concurrent mutable access нет.
