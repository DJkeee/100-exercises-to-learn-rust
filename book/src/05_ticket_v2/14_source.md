# `Error::source`

Чтобы завершить рассмотрение trait `Error`, осталось обсудить ещё одну тему: method `source`.

```rust
// На этот раз полное определение!
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
```

Метод `source` предоставляет доступ к **причине error**, если она существует.\
Errors часто объединяются в chain: один error является причиной другого. Например, high-level error
«невозможно подключиться к базе данных» может быть вызван low-level error «не удалось определить hostname базы данных».
Метод `source` позволяет «пройти» по всей chain errors, что часто используется при записи error context в logs.

## Implementation `source`

Trait `Error` предоставляет default implementation, которая всегда возвращает `None`, то есть underlying cause отсутствует.
Поэтому в предыдущих упражнениях о `source` можно было не беспокоиться.\
Эту default implementation можно override, указав причину для своего error type.

```rust
use std::error::Error;

#[derive(Debug)]
struct DatabaseError {
    source: std::io::Error
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to connect to the database")
    }
}

impl std::error::Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}
```

В этом примере `DatabaseError` оборачивает `std::io::Error`, используя его как source.
Затем мы override method `source`, чтобы он возвращал этот source при вызове.

## `&(dyn Error + 'static)`

Что представляет собой type `&(dyn Error + 'static)`?\
Разберём его по частям:

- `dyn Error` — это **trait object**. Он позволяет ссылаться на любой type, который implements trait `Error`.
- `'static` — особый **lifetime specifier**.
  `'static` означает, что reference допустима «столько, сколько потребуется», то есть на протяжении всего выполнения программы.

Вместе: `&(dyn Error + 'static)` — это reference на trait object, который implements trait `Error`
и допустим на протяжении всего выполнения программы.

Пока не стоит слишком беспокоиться об этих concepts. Мы подробнее рассмотрим их в следующих главах.

## Implementation `source` с помощью `thiserror`

`thiserror` предоставляет три способа автоматически implement `source` для error types:

- Поле с именем `source` автоматически используется как source error.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          source: std::io::Error
      }
  }
  ```
- Поле, помеченное attribute `#[source]`, автоматически используется как source error.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[source]
          inner: std::io::Error
      }
  }
  ```
- Поле, помеченное attribute `#[from]`, автоматически используется как source error, **а**
  `thiserror` автоматически сгенерирует implementation `From` для conversion помеченного type в ваш error type.
  ```rust
  use thiserror::Error;

  #[derive(Error, Debug)]
  pub enum MyError {
      #[error("Failed to connect to the database")]
      DatabaseError {
          #[from]
          inner: std::io::Error
      }
  }
  ```

## Operator `?`

Operator `?` — сокращённая запись для propagation errors.\
При использовании в function, возвращающей `Result`, он выполнит early return с error, если `Result` содержит `Err`.

Например:

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

эквивалентно:

```rust
use std::fs::File;

fn read_file() -> Result<String, std::io::Error> {
    let mut file = match File::open("file.txt") {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            return Err(e);
        }
    }
    Ok(contents)
}
```

Operator `?` позволяет значительно сократить code error handling.\
В частности, operator `?` автоматически преобразует error type fallible operation в error type
function, если такое conversion возможно, то есть если существует подходящая implementation `From`.
