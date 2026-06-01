// TODO: Реализуйте трейт `From` для типа `WrappingU32`, чтобы `example` компилировался.

pub struct WrappingU32 {
    value: u32,
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
