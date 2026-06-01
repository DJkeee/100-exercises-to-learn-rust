// TODO: Установите `move_forward` в `true` внутри `ready`, когда решите, что закончили упражнение.
//  Можете позвать преподавателя, чтобы проверить решение!
use channels::data::TicketDraft;
use channels::{launch, Command};
use std::time::Duration;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    sender
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
        // Если поток больше не выполняется, здесь возникнет паника,
        // поскольку канал будет закрыт.
        .expect("Did you actually spawn a thread? The channel is closed!");
}

#[test]
fn ready() {
    // В этом упражнении автоматически можно проверить совсем немногое,
    // поскольку наш сервер не предоставляет действий **чтения**.
    // У нас нет способа узнать, действительно ли выполняются вставки
    // и выполняются ли они правильно.
    let move_forward = false;

    assert!(move_forward);
}
