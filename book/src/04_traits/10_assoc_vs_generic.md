# Generics и associated types

Ещё раз рассмотрим определения двух изученных traits: `From` и `Deref`:

```rust
pub trait From<T> {
    fn from(value: T) -> Self;
}

pub trait Deref {
    type Target;
    
    fn deref(&self) -> &Self::Target;
}
```

В обоих присутствуют type parameters.\
В случае `From` это generic parameter `T`.\
В случае `Deref` это associated type `Target`.

В чём разница? Когда следует использовать один вариант, а когда другой?

## Не более одной implementation

Из-за устройства deref coercion у заданного type может быть только один target type. Например, `String` может
выполнять deref только в `str`.
Это позволяет избежать неоднозначности: если бы `Deref` можно было реализовать для type несколько раз,
какой type `Target` должен был бы выбрать compiler при вызове method с `&self`?

Именно поэтому `Deref` использует associated type `Target`.\
Associated type однозначно определяется **implementation trait**.
Поскольку реализовать `Deref` более одного раза нельзя, для заданного type можно указать только один `Target`,
и неоднозначности не возникнет.

## Generic traits

С другой стороны, `From` можно реализовать для type несколько раз, **если input type `T` различается**.
Например, можно реализовать `From` для `WrappingU32`, используя в качестве input types и `u32`, и `u16`:

```rust
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { inner: value }
    }
}

impl From<u16> for WrappingU32 {
    fn from(value: u16) -> Self {
        WrappingU32 { inner: value.into() }
    }
}
```

Это работает, потому что `From<u16>` и `From<u32>` считаются **разными traits**.\
Неоднозначности нет: compiler может определить нужную implementation по type преобразуемого value.

## Пример: `Add`

В заключение рассмотрим trait `Add` из standard library:

```rust
pub trait Add<RHS = Self> {
    type Output;
    
    fn add(self, rhs: RHS) -> Self::Output;
}
```

В нём используются оба механизма:

- generic parameter `RHS` (right-hand side), value которого по умолчанию является `Self`
- associated type `Output` — type результата сложения

### `RHS`

`RHS` — generic parameter, позволяющий складывать разные types.\
Например, в standard library можно найти две следующие implementations:

```rust
impl Add<u32> for u32 {
    type Output = u32;
    
    fn add(self, rhs: u32) -> u32 {
      //                      ^^^
      // Вместо этого можно написать `Self::Output`.
      // Компилятору все равно, если указанный здесь тип
      // совпадает с типом, который вы присвоили `Output`
      // чуть выше.
      // [...]
    }
}

impl Add<&u32> for u32 {
    type Output = u32;
    
    fn add(self, rhs: &u32) -> u32 {
        // [...]
    }
}
```

Благодаря этому компилируется следующий код:

```rust
let x = 5u32 + &5u32 + 6u32;
```

поскольку `u32` реализует как `Add<&u32>`, _так и_ `Add<u32>`.

### `Output`

`Output` представляет type результата сложения.

Зачем вообще нужен `Output`? Разве нельзя использовать в качестве output `Self`, то есть type, реализующий `Add`?
Можно, но это ограничит гибкость trait. Например, в standard library есть такая implementation:

```rust
impl Add<&u32> for &u32 {
    type Output = u32;

    fn add(self, rhs: &u32) -> u32 {
        // [...]
    }
}
```

Trait реализуется для type `&u32`, но результат сложения имеет type `u32`.\
Такая implementation была бы невозможна[^flexible], если бы `add` должен был возвращать `Self`, то есть в данном случае `&u32`.
`Output` позволяет `std` отделить implementor от return type и тем самым поддержать этот вариант.

С другой стороны, `Output` не может быть generic parameter. Когда types operands известны, output type операции
**должен** определяться однозначно. Поэтому он является associated type: для заданного сочетания implementor
и generic parameters существует только один type `Output`.

## Итог

Подведём итог:

- Используйте **associated type**, когда type должен однозначно определяться для заданной implementation trait.
- Используйте **generic parameter**, когда нужно разрешить несколько implementations trait для одного type
  с разными input types.

[^flexible]: Гибкость редко даётся бесплатно: из-за `Output` определение trait становится сложнее, а implementors приходится решать,
что именно возвращать. Такой компромисс оправдан лишь тогда, когда гибкость действительно нужна. Помните об этом,
проектируя собственные traits.
