# `Deref` trait

В предыдущем упражнении почти ничего не пришлось делать, верно?

Достаточно было заменить

```rust
impl Ticket {
    pub fn title(&self) -> &String {
        &self.title
    }
}
```

на

```rust
impl Ticket {
    pub fn title(&self) -> &str {
        &self.title
    }
}
```

чтобы код скомпилировался, а тесты прошли.
Но это должно вызывать вопросы.

## Это не должно работать, но работает

Рассмотрим факты:

- `self.title` имеет type `String`
- следовательно, `&self.title` имеет type `&String`
- output изменённого method `title` имеет type `&str`

Можно было бы ожидать ошибку compiler: `Expected &String, found &str` или что-то подобное.
Вместо этого всё просто работает. **Почему**?

## На помощь приходит `Deref`

Trait `Deref` лежит в основе возможности языка, известной как [**deref coercion**](https://doc.rust-lang.org/std/ops/trait.Deref.html#deref-coercion).\
Trait определён в standard library, в module `std::ops`:

```rust
// Пока я слегка упростил определение.
// Полное определение мы увидим позже.
pub trait Deref {
    type Target;
    
    fn deref(&self) -> &Self::Target;
}
```

`type Target` — это **associated type**.\
Это placeholder для concrete type, который необходимо указать при создании implementation trait.

## Deref coercion

Реализуя `Deref<Target = U>` для type `T`, вы сообщаете compiler, что `&T` и `&U`
в некоторой степени взаимозаменяемы.\
В частности, вы получаете следующее behavior:

- References на `T` неявно преобразуются в references на `U` (то есть `&T` превращается в `&U`)
- Для `&T` можно вызывать все methods, определённые для `U` и принимающие `&self` в качестве input.

С operator dereference `*` связан ещё один аспект, но пока он нам не нужен (если интересно,
обратитесь к documentation `std`).

## `String` реализует `Deref`

`String` реализует `Deref` с `Target = str`:

```rust
impl Deref for String {
    type Target = str;
    
    fn deref(&self) -> &str {
        // [...]
    }
}
```

Благодаря этой implementation и deref coercion value `&String` при необходимости автоматически преобразуется в `&str`.

## Не злоупотребляйте deref coercion

Deref coercion — мощная возможность, но она может внести путаницу.\
Автоматическое преобразование types способно усложнить чтение и понимание кода. Если method с одним и тем же именем
определён и для `T`, и для `U`, какой из них будет вызван?

Позже в курсе мы рассмотрим наиболее надёжные варианты использования deref coercion: smart pointers.
