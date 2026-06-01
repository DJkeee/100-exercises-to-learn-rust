/// TODO: Приведенный ниже код войдет во взаимную блокировку, поскольку использует каналы std,
///  которые не учитывают асинхронность.
///  Перепишите его с использованием примитива каналов `tokio` (да, тестовый код
///  тоже потребуется изменить).
///
/// Сможете понять последовательность событий, которая приводит к взаимной блокировке?
use std::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// Отвечает `pong` на любое полученное сообщение, создавая новый
/// канал для дальнейшего взаимодействия с вызывающим кодом.
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let Ok(msg) = receiver.recv() {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel();
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use std::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel();
        let (response_sender, response_receiver) = mpsc::channel();
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().unwrap().payload;
        assert_eq!(answer, "pong");
    }
}
