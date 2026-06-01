use crate::status::Status;

// Мы видели, как объявлять модули, в одном из первых упражнений, но
// еще не рассматривали, как выносить их в отдельные файлы.
// Исправим это сейчас!
//
// В простейшем случае, когда вынесенный модуль представляет собой один файл, достаточно
// создать новый файл с именем модуля и переместить туда содержимое модуля.
// Файл модуля должен находиться в том же каталоге, что и файл, объявляющий модуль.
// В данном случае это `src/lib.rs`, поэтому `status.rs` нужно поместить в каталог `src`.
mod status;

// TODO: Добавьте в `TicketNewError` новый вариант ошибки для недопустимой строки статуса.
//   При вызове `source` для ошибки этого варианта должен возвращаться `ParseStatusError`, а не `None`.

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // TODO: Преобразуйте строку статуса в перечисление `Status`.

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}
