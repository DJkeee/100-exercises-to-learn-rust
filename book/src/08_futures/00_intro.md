# Async Rust

Threads — не единственный способ писать concurrent-программы на Rust.\
В этой главе мы рассмотрим другой подход: **asynchronous programming**.

В частности, вы познакомитесь с:

- keywords `async`/`.await`, позволяющими без лишних усилий писать asynchronous code
- trait `Future`, представляющим вычисления, которые могут быть ещё не завершены
- `tokio`, самым популярным runtime для выполнения asynchronous code
- cooperative-природой asynchronous model Rust и её влиянием на ваш код
