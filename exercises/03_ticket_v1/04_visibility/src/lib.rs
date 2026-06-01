mod ticket {
    pub struct Ticket {
        pub title: String,
        pub description: String,
        pub status: String,
    }

    impl Ticket {
        fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }
    }
}

// TODO: **В виде исключения** в этом упражнении вы будете изменять и модуль `ticket`, и модуль `tests`.
#[cfg(test)]
mod tests {
    // TODO: Добавьте необходимые модификаторы `pub` в родительский модуль, чтобы устранить ошибки
    //  компилятора для приведенной ниже инструкции use.
    use super::ticket::Ticket;

    // Но будьте осторожны! Эта функция не должна компилироваться после того, как вы измените
    // видимость, чтобы инструкция use компилировалась!
    // Убедившись, что она действительно не компилируется, закомментируйте ее.
    //fn should_not_be_possible() {
      //  let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // При попытке запустить это упражнение вы должны увидеть следующую ошибку:
        //
        // error[E0616]: field `description` of struct `Ticket` is private
        //    |
        //    |              assert_eq!(ticket.description, "A description");
        //    |                         ^^^^^^^^^^^^^^^^^^
        //
        // TODO: Убедившись, что приведенная ниже строка не компилируется,
        //   закомментируйте ее, чтобы перейти к следующему упражнению!
        assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // Это тоже должно быть невозможно, с ошибкой, похожей на приведенную выше.
        // (Ошибка компиляции появится только после того, как вы закомментируете ошибочную строку
        // в предыдущем тесте — следующий этап компиляции!)
        //
        // Это доказывает, что теперь `Ticket::new` — единственный способ получить экземпляр `Ticket`.
        // Создать заявку с недопустимым заголовком или описанием невозможно!
        //
        // TODO: Убедившись, что приведенный ниже код не компилируется,
        //   закомментируйте строки, чтобы перейти к следующему упражнению!
        let ticket = Ticket {
            title: "A title".into(),
            description: "A description".into(),
            status: "To-Do".into(),
        };
    }
}
