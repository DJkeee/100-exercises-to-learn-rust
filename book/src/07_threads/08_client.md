# Отдельный type `Client`

До сих пор взаимодействие на стороне client было довольно низкоуровневым: требовалось
вручную создать response channel, сформировать command, отправить его server,
а затем вызвать `recv` у response channel, чтобы получить response.

Здесь много boilerplate code, который можно скрыть за abstraction.
Именно этим мы займёмся в упражнении.
