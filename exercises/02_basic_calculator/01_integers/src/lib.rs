fn compute(a: u32, b: u32) -> u32 {
    // TODO: измените строку ниже, чтобы исправить ошибку компилятора и обеспечить прохождение тестов.
    let multiplier: u32 = 4;
    a + b * multiplier
}

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
