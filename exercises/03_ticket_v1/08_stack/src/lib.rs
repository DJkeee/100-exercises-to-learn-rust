// TODO: Основываясь на изученном в этом разделе, замените `todo!()` правильным
//  **размером стека** для соответствующего типа.
#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        assert_eq!(size_of::<u16>(), todo!());
    }

    #[test]
    fn i32_size() {
        assert_eq!(size_of::<i32>(), todo!());
    }

    #[test]
    fn bool_size() {
        assert_eq!(size_of::<bool>(), todo!());
    }
}
