# Подведение итогов

Когда речь идёт о domain modelling, важны детали.\
Rust предлагает широкий набор инструментов, помогающих представить ограничения domain непосредственно в type system,
но для правильного и idiomatic codeа потребуется практика.

Завершим главу последним улучшением модели `Ticket`.\
Для каждого field `Ticket` введём новый type, инкапсулирующий соответствующие ограничения.\
При каждом обращении к field `Ticket` будет возвращаться гарантированно допустимое value, например
`TicketTitle` вместо `String`. В других частях codeа не придётся беспокоиться о том, что заголовок пуст:
наличие `TicketTitle` означает, что value допустимо **by construction**.

Это лишь один из примеров того, как type system Rust позволяет сделать code надёжнее и выразительнее.

## Дополнительные материалы

- [Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
- [Using types to guarantee domain invariants](https://www.lpalmieri.com/posts/2020-12-11-zero-to-production-6-domain-modelling/)
