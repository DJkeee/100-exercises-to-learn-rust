# Mutable references

Теперь ваши accessor methods должны выглядеть так:

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}
```

Несколько `&` тут и там решили задачу!\
Теперь у нас есть способ обращаться к fields instance `Ticket`, не потребляя его при этом.
Далее посмотрим, как дополнить нашу struct `Ticket` **методами-setter**.

## Setters

Setter methods позволяют пользователям изменять значения private fields `Ticket`, обеспечивая соблюдение его
инвариантов (то есть нельзя установить в качестве заголовка `Ticket` пустую строку).

В Rust есть два распространённых способа реализовать setters:

- Принимать `self` в качестве входного параметра.
- Принимать `&mut self` в качестве входного параметра.

### Принимаем `self` в качестве входного параметра

Первый подход выглядит так:

```rust
impl Ticket {
    pub fn set_title(mut self, new_title: String) -> Self {
        // Проверяем новый заголовок [...]
        self.title = new_title;
        self
    }
}
```

Он принимает ownership над `self`, изменяет заголовок и возвращает изменённый instance `Ticket`.\
Использовать его можно так:

```rust
let ticket = Ticket::new(
    "Title".into(), 
    "Description".into(), 
    "To-Do".into()
);
let ticket = ticket.set_title("New title".into());
```

Поскольку `set_title` принимает ownership над `self` (то есть **потребляет его**), нам нужно повторно присвоить результат variable.
В примере выше мы используем **variable shadowing**, чтобы сохранить прежнее имя variable: когда
вы объявляете новую variable с тем же именем, что и у существующей, новая variable **затеняет** старую. Это
распространённый паттерн в коде на Rust.

Setters с `self` удобны, когда нужно изменить сразу несколько полей: вызовы можно объединить в цепочку!

```rust
let ticket = ticket
    .set_title("New title".into())
    .set_description("New description".into())
    .set_status("In Progress".into());
```

### Принимаем `&mut self` в качестве входного параметра

Второй подход к setters, с использованием `&mut self`, выглядит так:

```rust
impl Ticket {
    pub fn set_title(&mut self, new_title: String) {
        // Проверяем новый заголовок [...]
        
        self.title = new_title;
    }
}
```

На этот раз method принимает mutable reference на `self`, изменяет заголовок, и на этом всё.
Он ничего не возвращает.

Использовать его можно так:

```rust
let mut ticket = Ticket::new(
    "Title".into(),
    "Description".into(),
    "To-Do".into()
);
ticket.set_title("New title".into());

// Используем измененную заявку
```

Ownership остаётся у caller, поэтому исходная variable `ticket` по-прежнему действительна. Повторно присваивать результат не нужно.
Но нужно объявить `ticket` mutable variable, поскольку мы берём mutable reference на неё.

У setters с `&mut` есть недостаток: объединить несколько вызовов в цепочку нельзя.
Поскольку они не возвращают изменённый instance `Ticket`, вызвать следующий setter для результата первого вызова не получится.
Каждый setter придётся вызывать отдельно:

```rust
ticket.set_title("New title".into());
ticket.set_description("New description".into());
ticket.set_status("In Progress".into());
```
