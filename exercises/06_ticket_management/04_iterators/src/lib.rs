use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Начнем набрасывать наше хранилище заявок!
//  Первое задание: реализуйте `IntoIterator` для `TicketStore`, чтобы можно было перебирать все
//  содержащиеся в нем заявки с помощью цикла `for`.
//
// Подсказка: в этом случае вам не потребуется реализовывать трейт `Iterator`.
//   Нужно *делегировать* итерацию полю `Vec<Ticket>` в `TicketStore`.
//   Найдите в документации стандартной библиотеки для `Vec` правильный тип,
//   возвращаемый из `into_iter`.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<_> = store.clone().into_iter().collect();
        assert_eq!(tickets, store.tickets);
    }
}
