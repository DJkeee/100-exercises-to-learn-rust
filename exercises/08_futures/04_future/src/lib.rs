//! TODO: Добейтесь компиляции кода, **изменив порядок** инструкций
//!  в функции `example`. Нельзя изменять функцию
//!  `spawner` или действия отдельных строк в `example`.
//!   При необходимости можно оборачивать существующие инструкции в блоки `{}`.
use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    let non_send = Rc::new(1);
    yield_now().await;
    println!("{}", non_send);
}
