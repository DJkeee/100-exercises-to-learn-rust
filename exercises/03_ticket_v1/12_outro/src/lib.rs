// TODO: Определите новый тип `Order`.
//   Он должен хранить три значения: `product_name`, `quantity` и `unit_price`.
//   Название товара не может быть пустым или длиннее 300 байт.
//   Количество должно быть строго больше нуля.
//   Цена единицы товара задается в центах и должна быть строго больше нуля.
//   В `Order` должен быть метод `total`, возвращающий общую стоимость заказа.
//   В `Order` должны быть сеттеры и геттеры для каждого поля.
//
// На этот раз тесты находятся в другом месте — в папке `tests`.
// Папка `tests` — особое место для `cargo`: именно там он ищет **интеграционные тесты**.
// Здесь слово «интеграционные» имеет конкретное значение: эти тесты проверяют **публичный API** проекта.
// Обратите внимание на видимость типов и методов: интеграционные тесты
// не имеют доступа к приватным элементам или элементам `pub(crate)`.


pub struct Order {
    quantity: u32,
    unit_price: u32,
    product_name: String,
}

impl Order {
    pub fn new( product_name: String, quantity: u32, unit_price: u32) -> Order {
        let mut order = Order {
            quantity: 0,
            unit_price: 0,
            product_name: String::new(),
        };
        order.set_quantity(quantity);
        order.set_unit_price(unit_price);
        order.set_product_name(product_name);
        order // возвращаем
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        assert!(quantity > 0);
        self.quantity = quantity;
    }
    pub fn set_unit_price(&mut self,unit_price: u32){
        assert!(unit_price > 0);
        self.unit_price = unit_price;
    }
    pub fn set_product_name(&mut self,product_name: String) {
        assert!(product_name.len() < 300);
        assert!(product_name != "");
        self.product_name = product_name;
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
}