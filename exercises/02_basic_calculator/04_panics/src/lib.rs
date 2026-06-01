/// Учитывая начальную и конечную точки путешествия, а также время, необходимое для его завершения,
/// вычислить среднюю скорость поездки.
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: Паника с собственным сообщением, если `time_elapsed` равен 0
    if time_elapsed == 0 {
        panic!("The journey took no time at all. That's impossible!");
    }
    (end - start) / time_elapsed

}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // 👇 С помощью аннотации `#[should_panic]` мы можем утверждать, что ожидаем код
    //    под испытанием на панику. Мы также можем проверить тревожное сообщение, используя `expected`.
    //    Все это часть встроенной среды тестирования Rust!
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
