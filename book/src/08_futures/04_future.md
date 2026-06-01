# Trait `Future`

## Проблема local `Rc`

Вернёмся к signature `tokio::spawn`:

```rust
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
{ /* */ }
```

Что _на самом деле_ означает требование `Send` для `F`?\
Как мы видели в предыдущем разделе, любое захваченное им value из spawning environment
должно быть `Send`. Но этим требование не ограничивается.

Любое value, _сохраняемое при прохождении через .await point_, должно быть `Send`.\
Рассмотрим пример:

```rust
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    // Значение без `Send`,
    // созданное _внутри_ асинхронной функции
    let non_send = Rc::new(1);
    
    // Точка `.await`, которая ничего не делает
    yield_now().await;

    // Локальное значение без `Send` все еще требуется
    // после `.await`
    println!("{}", non_send);
}
```

Compiler отклонит этот код:

```text
error: future cannot be sent between threads safely
    |
5   |     tokio::spawn(example());
    |                  ^^^^^^^^^ 
    | future returned by `example` is not `Send`
    |
note: future is not `Send` as this value is used across an await
    |
11  |     let non_send = Rc::new(1);
    |         -------- has type `Rc<i32>` which is not `Send`
12  |     // A `.await` point
13  |     yield_now().await;
    |                 ^^^^^ 
    |   await occurs here, with `non_send` maybe used later
note: required by a bound in `tokio::spawn`
    |
164 |     pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    |            ----- required by a bound in this function
165 |     where
166 |         F: Future + Send + 'static,
    |                     ^^^^ required by this bound in `spawn`
```

Чтобы понять причину, нужно уточнить наше представление об asynchronous model Rust.

## The `Future` trait

Ранее мы сказали, что `async` functions возвращают **futures** — types, реализующие
trait `Future`. Future можно представить как **state machine**.
Он находится в одном из двух states:

- **pending**: вычисление ещё не завершилось.
- **ready**: вычисление завершилось, результат готов.

Это отражено в definition trait:

```rust
trait Future {
    type Output;
    
    // Пока не обращайте внимания на `Pin` и `Context`
    fn poll(
      self: Pin<&mut Self>, 
      cx: &mut Context<'_>
    ) -> Poll<Self::Output>;
}
```

### `poll`

Method `poll` — основа trait `Future`.\
Сам по себе future ничего не делает. Для продвижения его необходимо **poll**.\
Вызывая `poll`, вы просите future выполнить некоторую работу.
`poll` пытается продвинуть вычисление и возвращает один из variants:

- `Poll::Pending`: future ещё не готов. Позднее потребуется снова вызвать `poll`.
- `Poll::Ready(value)`: future завершён. `value` — результат вычисления type `Self::Output`.

После возврата `Poll::Ready` из `Future::poll` future нельзя poll повторно:
он завершён, и выполнять больше нечего.

### The role of the runtime

Напрямую вы почти никогда не будете вызывать `poll`.\
Это задача async runtime: у него есть вся необходимая информация (`Context`
в signature `poll`), чтобы обеспечивать продвижение futures при любой возможности.

## `async fn` and futures

Мы работали с high-level interface — asynchronous functions.\
Теперь мы рассмотрели low-level primitive — trait `Future`.

Как они связаны?

Каждый раз, когда function помечается как asynchronous, она возвращает future.
Compiler преобразует тело asynchronous function в **state machine**:
по одному state для каждой `.await` point.

Вернёмся к примеру с `Rc`:

```rust
use std::rc::Rc;
use tokio::task::yield_now;

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
```

Compiler преобразует его в enum, примерно похожий на этот:

```rust
pub enum ExampleFuture {
    NotStarted,
    YieldNow(Rc<i32>),
    Terminated,
}
```

При вызове `example` возвращается `ExampleFuture::NotStarted`. Future ещё ни разу
не был poll, поэтому ничего не произошло.\
Когда runtime впервые выполняет poll, `ExampleFuture` продвигается до следующей
`.await` point: останавливается на этапе `ExampleFuture::YieldNow(Rc<i32>)`
state machine и возвращает `Poll::Pending`.\
При следующем poll он выполнит оставшийся код (`println!`) и вернёт `Poll::Ready(())`.

Если взглянуть на представление state machine `ExampleFuture`, становится понятно,
почему `example` не является `Send`: он хранит `Rc`, а значит, не может быть `Send`.

## Yield points

Как видно из примера `example`, каждая `.await` point создаёт новое промежуточное
state в lifecycle future.\
Поэтому `.await` points также называются **yield points**: future _передаёт управление_
выполнявшему poll runtime, позволяя приостановить его и при необходимости назначить
для выполнения другой task, concurrently продвигая несколько вычислений.

К важности yielding мы вернёмся в следующем разделе.
