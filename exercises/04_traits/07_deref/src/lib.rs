// TODO: При возврате `title` и `description` через методы доступа нормализуйте их:
//   удаляйте начальные и конечные пробельные символы.
//   В стандартной библиотеке Rust есть метод, который поможет это сделать, но вы не найдете
//   его в документации для `String`.
//   Сможете определить, где он объявлен и как его использовать?

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        todo!()
    }

    pub fn description(&self) -> &str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
