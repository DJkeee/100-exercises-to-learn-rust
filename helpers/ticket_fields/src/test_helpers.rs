use crate::{TicketDescription, TicketTitle};
use common::{valid_description, valid_title};

/// Функция для генерации допустимого заголовка заявки
/// в целях тестирования.
pub fn ticket_title() -> TicketTitle {
    valid_title().try_into().unwrap()
}

/// Функция для генерации допустимого описания заявки
/// в целях тестирования.
pub fn ticket_description() -> TicketDescription {
    valid_description().try_into().unwrap()
}
