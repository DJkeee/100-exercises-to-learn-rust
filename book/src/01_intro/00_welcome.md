# Добро пожаловать

Добро пожаловать в курс **"100 Exercises To Learn Rust"**!

В этом курсе вы будете изучать ключевые концепции Rust, выполняя одно упражнение за другим.\
Вы познакомитесь с syntax Rust, его type system, standard library и ecosystem.

Мы не предполагаем, что вы уже знакомы с Rust, но рассчитываем, что вы знаете хотя бы
один другой язык программирования.
Также от вас не требуются предварительные знания о systems programming или memory management. Эти
темы будут рассмотрены в курсе.

Иными словами, мы начнём с нуля!\
Вы будете осваивать Rust небольшими, посильными шагами.
К концу курса вы решите около 100 упражнений. Этого достаточно, чтобы
уверенно работать с небольшими и средними проектами на Rust.

## Методика

Этот курс основан на принципе «обучение через практику».\
Он задуман как интерактивный практический курс.

[Mainmatter](https://mainmatter.com/rust-consulting/) разработала этот курс
для проведения в аудитории в течение 4 дней: каждый участник проходит
уроки в своём темпе, а опытный преподаватель помогает,
отвечает на вопросы и при необходимости подробнее разбирает темы.\
Записаться на следующее занятие с преподавателем можно на [нашем сайте](https://ti.to/mainmatter/rust-from-scratch-jan-2025).
Если вы хотите организовать отдельное занятие для своей компании, пожалуйста, [свяжитесь с нами](https://mainmatter.com/contact/).

Вы также можете пройти курс самостоятельно, но мы рекомендуем найти друга или
ментора, который поможет, если вы столкнётесь с трудностями. Решения всех упражнений
находятся в
[`solutions` branch GitHub repository](https://github.com/mainmatter/100-exercises-to-learn-rust/tree/solutions).

## Форматы

Вы можете изучать материал курса [в browser](https://rust-exercises.com/100-exercises/) или [скачать его как PDF file](https://rust-exercises.com/100-exercises-to-learn-rust.pdf) для чтения offline.\
Если вы предпочитаете печатный вариант, [приобретите бумажную копию на Amazon](https://www.amazon.com/dp/B0DJ14KQQG/).

## Структура

В левой части экрана видно, что курс разделён на разделы.
Каждый раздел знакомит с новой концепцией или возможностью языка Rust.\
Чтобы проверить понимание материала, к каждому разделу прилагается упражнение, которое нужно решить.

Упражнения находятся в
[сопутствующем GitHub repository](https://github.com/mainmatter/100-exercises-to-learn-rust).\
Перед началом курса обязательно выполните clone repository на локальный компьютер:

```bash
# Если у вас настроен SSH-ключ для GitHub
git clone git@github.com:mainmatter/100-exercises-to-learn-rust.git
# В противном случае используйте HTTPS URL:
#   https://github.com/mainmatter/100-exercises-to-learn-rust.git
```

Также рекомендуем работать в отдельном branch, чтобы было проще отслеживать свой прогресс и при необходимости получать
обновления из основного repository:

```bash
cd 100-exercises-to-learn-rust
git checkout -b my-solutions
```

Все упражнения находятся в folder `exercises`.
Каждое упражнение оформлено как Rust package.
Package содержит само упражнение, инструкции по его выполнению (в `src/lib.rs`) и test suite для
автоматической проверки решения.

### Инструменты

Для прохождения курса вам понадобятся:

- [**Rust**](https://www.rust-lang.org/tools/install).
  Если в вашей системе уже установлен `rustup`, выполните `rustup update` (или другую подходящую команду в зависимости от способа установки Rust), чтобы использовать последнюю stable version.
- _(Необязательно, но рекомендуется)_ IDE с поддержкой autocompletion для Rust.
  Мы рекомендуем один из следующих вариантов:
  - [RustRover](https://www.jetbrains.com/rust/);
  - [Visual Studio Code](https://code.visualstudio.com) с extension [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer).

### Workshop runner, `wr`

Для проверки решений мы также предоставили инструмент, который будет сопровождать вас на протяжении курса: `wr` CLI, сокращение от "workshop runner".
Установите `wr`, следуя инструкциям на [его сайте](https://mainmatter.github.io/rust-workshop-runner/).

После установки `wr` откройте новый terminal и перейдите в top-level folder repository.
Чтобы начать курс, выполните команду `wr`:

```bash
wr
```

`wr` проверит решение текущего упражнения.\
Не переходите к следующему разделу, пока не решите упражнение для текущего.

> По мере прохождения курса рекомендуем выполнять commit решений в Git,
> чтобы легко отслеживать прогресс и при необходимости «перезапуститься» с известной точки.

Приятного обучения!

## Автор

Этот курс написал [Luca Palmieri](https://www.lpalmieri.com/), Principal Engineering
Consultant в [Mainmatter](https://mainmatter.com/rust-consulting/).\
Luca работает с Rust с 2018 года: сначала в TrueLayer, затем в AWS.\
Luca является автором книги ["Zero to Production in Rust"](https://zero2prod.com),
одного из основных ресурсов для изучения разработки backend applications на Rust.\
Кроме того, он автор и maintainer ряда open-source проектов на Rust, включая
[`cargo-chef`](https://github.com/LukeMathWalker/cargo-chef),
[Pavex](https://pavex.dev) и [`wiremock`](https://github.com/LukeMathWalker/wiremock-rs).
