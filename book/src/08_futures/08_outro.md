# Заключение

Asynchronous model Rust весьма мощная, но добавляет сложности. Уделите время изучению
инструментов: внимательно прочитайте documentation `tokio` и познакомьтесь с его
primitives, чтобы использовать его возможности наиболее эффективно.

Помните также, что на уровне языка и `std` продолжается работа над упрощением и завершением
asynchronous story Rust. Из-за некоторых недостающих частей в повседневной работе
могут встречаться неудобства.

Несколько рекомендаций для более комфортной работы с async:

- **Выберите runtime и придерживайтесь его.**\
  Некоторые primitives (например, timers и I/O) нельзя переносить между runtimes. Попытка
  смешивать runtimes, скорее всего, создаст проблемы. Runtime-agnostic code может значительно
  увеличить сложность codebase. По возможности избегайте его.
- **Stable interface `Stream`/`AsyncIterator` пока отсутствует.**\
  Концептуально `AsyncIterator` — iterator, который asynchronous выдаёт новые элементы.
  Работа над design продолжается, но consensus пока отсутствует.
  При использовании `tokio` основным interface должен быть
  [`tokio_stream`](https://docs.rs/tokio-stream/latest/tokio_stream/).
- **Будьте осторожны с buffering.**\
  Он часто становится причиной трудноуловимых bugs. Подробнее см.
  ["Barbara battles buffered streams"](https://rust-lang.github.io/wg-async/vision/submitted_stories/status_quo/barbara_battles_buffered_streams.html).
- **Для asynchronous tasks нет аналога scoped threads**.\
  Подробнее см. ["The scoped task trilemma"](https://without.boats/blog/the-scoped-task-trilemma/).

Пусть эти оговорки вас не пугают: asynchronous Rust эффективно используется
в _огромных_ масштабах (например, в AWS и Meta) для работы ключевых services.\
Вам придётся освоить его, если вы планируете создавать networked applications на Rust.
