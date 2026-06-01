// Настройте профиль `dev` для переноса при переполнении.
// Проверьте документацию Cargo, чтобы узнать правильный синтаксис:
// https://doc.rust-lang.org/cargo/reference/profiles.html
//
// По причинам, которые мы объясним позже, настройку необходимо выполнить в `Cargo.toml`.
// в корне репозитория, а не в `Cargo.toml` упражнения.

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! это 2432902008176640000, что слишком велико для u32.
        // При использовании профиля разработчика по умолчанию при запуске `cargo test` возникнет паника.
        // Вместо этого мы хотим, чтобы он обертывался
        assert_eq!(factorial(20), 2_192_834_560);
        //                           ☝️
        // Большой числовой литерал с использованием подчеркивания для улучшения читаемости!
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
