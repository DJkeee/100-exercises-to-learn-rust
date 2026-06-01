# Iteration

В самых первых упражнениях вы узнали, что Rust позволяет выполнять iteration по collections с помощью циклов `for`.
Тогда мы рассматривали ranges (например, `0..5`), но то же самое справедливо для таких collections, как arrays и vectors.

```rust
// Работает для `Vec`
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}

// Также работает для массивов
let a: [u32; 3] = [1, 2, 3];
for n in a {
    println!("{}", n);
}
```

Пора разобраться, как это работает внутри.

## Desugaring цикла `for`

Каждый раз, когда вы пишете в Rust цикл `for`, compiler выполняет его _desugaring_ в следующий код:

```rust
let mut iter = IntoIterator::into_iter(v);
loop {
    match iter.next() {
        Some(n) => {
            println!("{}", n);
        }
        None => break,
    }
}
```

`loop` — ещё одна конструкция цикла наряду с `for` и `while`.\
Блок `loop` выполняется бесконечно, если явно не выйти из него с помощью `break`.

## Trait `Iterator`

Метод `next` из предыдущего фрагмента кода предоставляется trait `Iterator`.
Trait `Iterator` определён в standard library Rust и предоставляет общий interface для
types, способных выдавать последовательность значений:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Associated type `Item` задаёт type значений, выдаваемых iterator.

`next` возвращает следующее значение последовательности.\
Он возвращает `Some(value)`, если значение есть, и `None`, если его нет.

Будьте внимательны: если iterator вернул `None`, это ещё не гарантирует, что он исчерпан. Такая гарантия
предоставляется только в том случае, если iterator реализует более строгий trait
[`FusedIterator`](https://doc.rust-lang.org/std/iter/trait.FusedIterator.html).

## Trait `IntoIterator`

Не все types реализуют `Iterator`, но многие можно преобразовать в type, который его реализует.\
Для этого и нужен trait `IntoIterator`:

```rust
trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;
    fn into_iter(self) -> Self::IntoIter;
}
```

Метод `into_iter` consumes исходное значение и возвращает iterator по его элементам.\
У type может быть только одна implementation `IntoIterator`: не должно быть неоднозначности в том, во что выполнять desugaring цикла `for`.

Ещё одна деталь: каждый type, реализующий `Iterator`, автоматически реализует и `IntoIterator`.
Такие types просто возвращают самих себя из `into_iter`!

## Bounds checks

Iteration с помощью iterators даёт полезный побочный эффект: выйти за границы невозможно по определению.\
Благодаря этому Rust может исключить bounds checks из сгенерированного машинного кода и ускорить iteration.

Иными словами,

```rust
let v = vec![1, 2, 3];
for n in v {
    println!("{}", n);
}
```

обычно работает быстрее, чем

```rust
let v = vec![1, 2, 3];
for i in 0..v.len() {
    println!("{}", v[i]);
}
```

У этого правила есть исключения: иногда compiler может доказать, что выхода за границы не будет даже
при ручном indexing, и всё равно удалить bounds checks. Но в целом там, где это возможно, предпочитайте iteration
вместо indexing.
