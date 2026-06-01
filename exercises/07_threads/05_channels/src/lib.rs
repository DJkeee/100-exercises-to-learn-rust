use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(todo!()),
}

// Запускает систему, создавая серверный поток.
// Возвращает экземпляр `Sender`, который затем могут использовать
// один или несколько клиентов для взаимодействия с сервером.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: Серверная задача **никогда** не должна останавливаться.
//  Запустите цикл: дождитесь появления команды
//  в канале, выполните ее, затем начинайте ожидать
//  следующую команду.
pub fn server(receiver: Receiver<Command>) {}
