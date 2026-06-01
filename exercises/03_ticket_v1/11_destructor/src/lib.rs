// Чтобы написать полноценное упражнение о деструкторах, нам понадобится дополнительный инструментарий.
// Мы вернемся к этой концепции в одной из следующих глав после изучения трейтов и
// внутренней изменяемости.
fn outro() -> &'static str {
    "I have a basic understanding of __!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
