// Определите структуру с именем `Order` со следующими полями:
// - `price`, целое число без знака
// - `quantity`, целое число без знака
//
// Он также должен иметь метод с именем `is_available`, который возвращает `true`, если количество
// больше 0, иначе `false`.

struct Order {
    price: u32,
    quantity: u32,
}

impl Order {
    fn is_available(&self) -> bool {
        if self.quantity > 0 {
            return true
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}
