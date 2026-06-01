# Two-way communication

В текущей client-server implementation communication направлена в одну сторону: от client к server.\
Client не может узнать, получил ли server message, успешно ли обработал его или столкнулся с error.
Это не лучший вариант.

Для решения проблемы можно добавить систему two-way communication.

## Response channel

Нужен способ отправить response от server обратно client.\
Есть разные решения, но самое простое — включить channel `Sender` в message,
которое client отправляет server. После обработки message server сможет использовать
этот channel для отправки response обратно client.

Это довольно распространённый pattern в приложениях Rust, построенных на primitives message passing.
