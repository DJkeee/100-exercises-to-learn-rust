# Обзор design

Подведём итоги пройденного пути.

## Lockless-подход с serialization через channel

Первая implementation multithreaded-хранилища заявок использовала:

- один long-lived thread (server), хранящий shared state
- несколько clients, отправляющих ему requests через channels из собственных threads.

Locking state не требовался, поскольку state изменял только server. Это возможно благодаря тому, что
«входящий» channel естественным образом **serialized** поступающие requests: server обрабатывал их по очереди.\
Мы уже обсудили ограничения такого подхода применительно к patching behavior, но не рассмотрели влияние исходного
design на performance: server мог обрабатывать только один request за раз, включая read requests.

## Fine-grained locking

Затем мы перешли к более сложному design: каждая заявка защищена собственным lock,
а clients независимо решают, читать или atomically изменять заявку, выполняя acquire соответствующего lock.

Этот design обеспечивает лучший parallelism, то есть несколько clients могут одновременно читать заявки, но в основе
по-прежнему остаётся **serial**-подход: server обрабатывает commands по очереди. В частности, он выдаёт locks clients
один за другим.

Можно ли полностью убрать channels и разрешить clients напрямую обращаться к `TicketStore`, используя только
locks для synchronization доступа?

## Removing channels

Нужно решить две проблемы:

- Sharing `TicketStore` между threads
- Synchronization доступа к хранилищу

### Sharing `TicketStore` между threads

Все threads должны обращаться к одному state, иначе у нас не будет настоящей multithreaded-системы:
мы просто запустим несколько single-threaded-систем параллельно.\
Мы уже сталкивались с этой проблемой при попытке share lock между threads: можно использовать `Arc`.

### Synchronization доступа к хранилищу

Благодаря serialization, обеспечиваемой channels, одна operation всё ещё остаётся lockless: insertion заявки
в хранилище или её removal.\
Если убрать channels, понадобится добавить ещё один lock для synchronization доступа к самому `TicketStore`.

При использовании `Mutex` нет смысла добавлять `RwLock` для каждой заявки: `Mutex` и так будет serialize доступ
ко всему хранилищу, поэтому параллельно читать заявки всё равно не получится.\
А при использовании `RwLock` заявки можно читать параллельно. Нужно лишь приостанавливать все read operations
на время insertion или removal заявки.

Пойдём этим путём и посмотрим, к чему он приведёт.
