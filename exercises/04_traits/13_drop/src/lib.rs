// TODO: Реализуйте так называемую "Drop bomb": тип, вызывающий панику при удалении,
//  если над ним не была выполнена определенная операция.
//  Ожидаемый API показан в тестах ниже.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // Бомба должна вызвать панику при удалении
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // Бомба не должна вызвать панику при удалении,
        // поскольку она обезврежена
    }
}
