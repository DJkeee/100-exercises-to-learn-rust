# Derive macros

Реализовывать `PartialEq` для `Ticket` было несколько утомительно, не так ли?
Пришлось вручную сравнивать каждый field struct.

## Синтаксис destructuring

Кроме того, implementation хрупка: если определение struct изменится
(например, будет добавлен новый field), нужно не забыть обновить implementation `PartialEq`.

Риск можно снизить, выполнив **destructuring** struct на fields:

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        // [...]
    }
}
```

Если определение `Ticket` изменится, compiler сообщит об ошибке: destructuring больше не является exhaustive.\
Fields struct также можно переименовать, чтобы избежать variable shadowing:

```rust
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        let Ticket {
            title,
            description,
            status,
        } = self;
        let Ticket {
            title: other_title,
            description: other_description,
            status: other_status,
        } = other;
        // [...]
    }
}
```

Destructuring — полезный pattern, который стоит иметь в своём арсенале, но
есть ещё более удобный способ: **derive macros**.

## Macros

В предыдущих упражнениях вам уже встречались несколько macros:

- `assert_eq!` и `assert!` в тестах
- `println!` для вывода в console

Macros Rust — это **code generators**.\
Они генерируют новый код Rust на основе переданных input, после чего созданный код компилируется вместе
с остальной программой. Некоторые macros встроены в standard library Rust, но можно
писать и собственные. В этом курсе мы не будем создавать свои macros, однако полезные
ссылки приведены в [разделе «Дополнительные материалы»](#further-reading).

### Изучение

Некоторые IDE позволяют раскрыть macro и изучить сгенерированный код. Если такой возможности нет, используйте
[`cargo-expand`](https://github.com/dtolnay/cargo-expand).

### Derive macros

**Derive macro** — особая разновидность macro Rust. Она задаётся как **attribute** над struct.

```rust
#[derive(PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String
}
```

Derive macros используются для автоматизации implementation распространённых (и «очевидных») traits для пользовательских types.
В примере выше trait `PartialEq` автоматически реализуется для `Ticket`.
Если раскрыть macro, можно увидеть, что сгенерированный код функционально эквивалентен написанному вручную,
хотя читать его несколько сложнее:

```rust
#[automatically_derived]
impl ::core::cmp::PartialEq for Ticket {
    #[inline]
    fn eq(&self, other: &Ticket) -> bool {
        self.title == other.title 
            && self.description == other.description
            && self.status == other.status
    }
}
```

Когда это возможно, compiler предложит использовать derive для traits.

## Дополнительные материалы

- [The little book of Rust macros](https://veykril.github.io/tlborm/)
- [Proc macro workshop](https://github.com/dtolnay/proc-macro-workshop)
