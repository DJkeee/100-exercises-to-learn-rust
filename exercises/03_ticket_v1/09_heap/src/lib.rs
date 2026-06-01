pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: Основываясь на изученном в этом разделе, замените `todo!()` правильным
//  **размером стека** для соответствующего типа.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), todo!());
    }

    #[test]
    fn ticket_size() {
        // Это непростой вопрос!
        // На этот раз «интуитивный» ответ оказывается правильным,
        // но в общем случае размещение структур в памяти — более сложная тема.
        // Если вам интересно, дополнительную информацию можно найти в разделе "Type layout"
        // справочника The Rust Reference: https://doc.rust-lang.org/reference/type-layout.html.
        assert_eq!(size_of::<Ticket>(), todo!());
    }
}
