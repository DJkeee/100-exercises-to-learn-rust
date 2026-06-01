# Overflow

Факториал числа растёт довольно быстро.\
Например, факториал 20 равен 2,432,902,008,176,640,000. Это уже больше максимального значения
32-bit integer: 2,147,483,647.

Если результат arithmetic operation превышает максимальное значение заданного integer type,
возникает **integer overflow**.

Integer overflows представляют проблему, поскольку нарушают контракт arithmetic operations.\
Результатом arithmetic operation над двумя integers заданного type должен быть ещё один integer того же type.
Но _математически правильный результат_ не помещается в этот integer type!

> Если результат меньше минимального значения заданного integer type, возникает **integer
> underflow**.\
> Для краткости в оставшейся части раздела мы будем говорить только об integer overflows, но имейте в виду:
> всё сказанное относится и к integer underflows.
>
> Function `speed`, которую вы написали в разделе ["Variables"](02_variables.md), приводит к underflow при некоторых комбинациях
> input.
> Например, если `end` меньше `start`, выражение `end - start` вызовет underflow type `u32`, поскольку результат должен
> быть отрицательным, а `u32` не может представлять отрицательные числа.

## Без автоматического promotion

Один из возможных подходов — автоматически выполнять promotion результата до более крупного integer type.
Например, если сложить два integers `u8` и получить 256 (`u8::MAX + 1`), Rust мог бы интерпретировать
результат как `u16`: следующий integer type, в который помещается 256.

Но, как мы уже обсуждали, Rust довольно строго относится к type conversions. Automatic integer promotion
не является решением проблемы integer overflow в Rust.

## Альтернативы

Поскольку automatic promotion исключён, что можно сделать при возникновении integer overflow?\
Есть два основных подхода:

- Отклонить operation
- Подобрать «разумный» результат, который помещается в ожидаемый integer type

### Отклонить operation

Это самый консервативный подход: при возникновении integer overflow программа останавливается.\
Для этого используется panic, механизм, с которым мы уже познакомились в разделе ["Panics"](04_panics.md).

### Подобрать «разумный» результат

Если результат arithmetic operation превышает максимальное значение заданного integer type, можно
выбрать **wrapping**.\
Если представить все допустимые значения заданного integer type расположенными по кругу, wrapping означает, что после
достижения максимального значения отсчёт снова начинается с минимального.

Например, результат **wrapping addition** значений 1 и 255 (=`u8::MAX`) равен 0 (=`u8::MIN`).
Для signed integers действует тот же принцип. Например, прибавление 1 к 127 (=`i8::MAX`) с wrapping
даст -128 (=`i8::MIN`).

## `overflow-checks`

Rust позволяет developer выбрать подход, используемый при возникновении integer overflow.
Поведение управляется setting profile `overflow-checks`.

Если `overflow-checks` имеет значение `true`, Rust вызовет **panic at runtime** при overflow integer operation.
Если `overflow-checks` имеет значение `false`, Rust применит **wrapping** при overflow integer operation.

Возможно, вы задаётесь вопросом: что такое setting profile? Давайте разберёмся!

## Profiles

[**Profile**](https://doc.rust-lang.org/cargo/reference/profiles.html) — это набор configuration options, позволяющих
настроить способ compilation code Rust.

Cargo предоставляет 4 встроенных profiles: `dev`, `release`, `test` и `bench`.\
Profile `dev` используется при каждом запуске `cargo build`, `cargo run` или `cargo test`. Он предназначен для локальной
разработки,
поэтому жертвует runtime performance ради более быстрой compilation и удобной debugging.\
Profile `release`, напротив, оптимизирован для runtime performance, но требует больше времени на compilation. Его нужно
явно запрашивать с помощью flag `--release`, например: `cargo build --release` или `cargo run --release`.
Profile `test` используется по умолчанию командой `cargo test`. Profile `test` наследует settings profile `dev`.
Profile `bench` используется по умолчанию командой `cargo bench`. Profile `bench` наследует settings profile `release`.
Используйте `dev` для iterative development и debugging, `release` для оптимизированных production builds,\
`test` для проверки корректности, а `bench` для performance benchmarking.

> "Have you built your project in release mode?" — почти мем в Rust community.\
> Речь идёт о developers, которые ещё не знакомы с Rust и жалуются на его performance в
> социальных сетях (например, Reddit или Twitter), прежде чем понимают, что не выполнили build проекта в
> release mode.

Можно также определять собственные profiles или настраивать встроенные.

### `overflow-check`

По умолчанию для `overflow-checks` установлены следующие значения:

- `true` для profile `dev`
- `false` для profile `release`

Это соответствует назначению двух profiles.\
`dev` предназначен для локальной разработки, поэтому вызывает panic, чтобы как можно раньше выявлять потенциальные проблемы.\
`release`, напротив, настроен для runtime performance: проверки overflow замедлили бы программу, поэтому он
предпочитает wrapping.

В то же время различное поведение двух profiles может приводить к трудноуловимым bugs.\
Мы рекомендуем включить `overflow-checks` для обоих profiles: лучше завершить работу с ошибкой, чем незаметно получить
неверный результат. В большинстве случаев влияние на runtime performance пренебрежимо мало. Если вы работаете над
performance-critical application, выполните benchmarks и решите, допустимы ли эти затраты.

## Дополнительные материалы

- Прочитайте ["Myths and legends about integer overflow in Rust"](https://huonw.github.io/blog/2016/04/myths-and-legends-about-integer-overflow-in-rust/)
  для подробного обсуждения integer overflow в Rust.
