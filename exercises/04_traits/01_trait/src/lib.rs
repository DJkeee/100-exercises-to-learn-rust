// Определите трейт `IsEven` с методом `is_even`, который возвращает `true`, если `self`
// является четным числом, и `false` в противном случае.
//
// Затем реализуйте трейт для `u32` и `i32`.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u32.is_even());
        assert!(!43u32.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
