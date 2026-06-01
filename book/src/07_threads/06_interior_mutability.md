# Interior mutability

Рассмотрим signature method `send` у `Sender`:

```rust
impl<T> Sender<T> {
    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        // [...]
    }
}
```

`send` принимает `&self` как argument.\
Но он явно выполняет mutation: добавляет новое message в channel.
Что ещё интереснее, `Sender` является cloneable: у нас может быть несколько instances `Sender`,
которые пытаются изменить state channel **одновременно** из разных threads.

Именно это свойство используется для построения client-server architecture. Но почему это работает?
Разве это не нарушает правила Rust для borrowing? Как выполняются mutations через _immutable_ reference?

## Shared references, а не immutable references

Когда мы знакомились с borrow checker, то назвали два вида references в Rust:

- immutable references (`&T`)
- mutable references (`&mut T`)

Точнее было бы назвать их:

- shared references (`&T`)
- exclusive references (`&mut T`)

Модель immutable/mutable подходит для подавляющего большинства случаев и отлично помогает начать работу
с Rust. Но, как вы только что увидели, она не описывает картину полностью: `&T` на самом деле не гарантирует
immutable-характер data, на которые указывает.\
При этом Rust по-прежнему выполняет свои обещания.
Просто смысл терминов несколько тоньше, чем может показаться сначала.

## `UnsafeCell`

Если type позволяет mutate data через shared reference, перед вами **interior mutability**.

По умолчанию compiler Rust считает shared references immutable и **оптимизирует ваш code** с учётом этого предположения.\
Compiler может менять порядок operations, cache values и выполнять другие преобразования для ускорения code.

Можно сообщить compiler: «Нет, этот shared reference на самом деле mutable», обернув data в `UnsafeCell`.\
Всякий раз, когда встречается type, допускающий interior mutability, можно быть уверенным, что прямо
или косвенно используется `UnsafeCell`.\
С помощью `UnsafeCell`, raw pointers и `unsafe` code data можно mutate через shared references.

Важно понимать: `UnsafeCell` не позволяет игнорировать borrow checker!\
На `unsafe` code по-прежнему распространяются правила Rust для borrowing и aliasing.
Это продвинутый инструмент для создания **safe abstractions**, безопасность которых невозможно выразить напрямую
в type system Rust. Используя keyword `unsafe`, вы сообщаете compiler:
«Я знаю, что делаю, и не нарушу твои invariants, доверься мне».

У каждой `unsafe` function есть документация с описанием её **safety preconditions**:
условий, при которых выполнение её `unsafe` block безопасно. Условия для `UnsafeCell` приведены
[в документации `std`](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html).

В этом курсе мы не будем использовать `UnsafeCell` напрямую и писать `unsafe` code.
Но важно знать о его существовании, назначении и связи с types, которые вы ежедневно используете
в Rust.

## Основные примеры

Рассмотрим несколько важных types из `std`, использующих interior mutability.\
Эти types довольно часто встречаются в Rust code, особенно если заглядывать во внутреннее устройство
используемых libraries.

### Reference counting

`Rc` — reference-counted pointer.\
Он оборачивает value и отслеживает количество существующих references на него.
Когда последний reference dropped, value deallocated.\
Value внутри `Rc` immutable: получить на него можно только shared references.

```rust
use std::rc::Rc;

let a: Rc<String> = Rc::new("My string".to_string());
// Существует только одна ссылка на строковые данные.
assert_eq!(Rc::strong_count(&a), 1);

// При вызове `clone` строковые данные не копируются!
// Вместо этого увеличивается счетчик ссылок `Rc`.
let b = Rc::clone(&a);
assert_eq!(Rc::strong_count(&a), 2);
assert_eq!(Rc::strong_count(&b), 2);
// ^ И `a`, и `b` указывают на одни и те же строковые данные
//   и используют общий счетчик ссылок.
```

Внутри `Rc` используется `UnsafeCell`, чтобы shared references могли увеличивать и уменьшать reference count.

### `RefCell`

`RefCell` — один из наиболее распространённых примеров interior mutability в Rust.
Он позволяет mutate value внутри `RefCell`, даже если у вас есть только
immutable reference на сам `RefCell`.

Это реализовано через **runtime borrow checking**.
Во время runtime `RefCell` отслеживает количество и вид references на содержащееся в нём value.
Если попытаться mutably borrow value, которое уже immutably borrowed,
программа вызовет panic, обеспечивая соблюдение правил Rust для borrowing.

```rust
use std::cell::RefCell;

let x = RefCell::new(42);

let y = x.borrow(); // Неизменяемое заимствование
let z = x.borrow_mut(); // Паника! Существует активное неизменяемое заимствование.
```
