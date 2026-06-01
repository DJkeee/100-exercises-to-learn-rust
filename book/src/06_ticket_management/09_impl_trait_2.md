# `impl Trait` в argument position

В предыдущем разделе мы увидели, как с помощью `impl Trait` вернуть type, не указывая его имя.\
Тот же синтаксис можно использовать и в **argument position**:

```rust
fn print_iter(iter: impl Iterator<Item = i32>) {
    for i in iter {
        println!("{}", i);
    }
}
```

`print_iter` принимает iterator значений `i32` и выводит каждый элемент.\
При использовании в **argument position** `impl Trait` эквивалентен generic parameter с trait bound:

```rust
fn print_iter<T>(iter: T) 
where
    T: Iterator<Item = i32>
{
    for i in iter {
        println!("{}", i);
    }
}
```

## Недостатки

Как правило, в argument position следует предпочитать generics вместо `impl Trait`.\
Generics позволяют caller явно указать тип аргумента с помощью turbofish syntax (`::<>`),
что может быть полезно для устранения неоднозначности. `impl Trait` такой возможности не даёт.
