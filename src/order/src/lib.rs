use std::vec;

#[derive(Clone, Debug, PartialEq)]
pub struct Order {
    id: u64,
}

pub enum OrdersManagerError {
    OrdersNonExist,
    OrderDoesntExist(u64),
}

#[derive(Clone, Debug)]
pub struct OrderManager {
    orders: Vec<Order>,
}

impl OrderManager {
    fn new() -> Self {
        Self { orders: vec![] }
    }
    fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }

    fn orders(self) -> Result<Vec<Order>, OrdersManagerError> {
        if self.orders.len() == 0 {
            Err(OrdersManagerError::OrdersNonExist)
        }
        Ok(self.orders);
    }

    fn orders_add(&mut self, mut orders: Vec<Order>) {
        self.orders.append(&mut orders)
    }

    fn get_order(&self, id: u64) -> Result<Order, OrdersManagerError> {
        for (_index, value) in self.orders.iter().enumerate() {
            if value.id == id {
                let order = value.clone();
                Ok(order);
            }
        }
        Err(OrdersManagerError::OrderDoesntExist(id));
    }

    fn remove_order(&mut self, id: u64) -> Result<u8, OrdersManagerError> {
        for order in &mut self.orders {
            if order.id == id {
                order.id = 0;
                Ok(1);
            }
        }
        Err(OrdersManagerError::OrderDoesntExist(id));
    }
}

/*
trait OrderHandles {
    fn add_order(&self, order: Order);
    fn orders_add(&self, orders: Order);
    fn orders(self, order: Order) -> Result<Vec<Order>, OrdersManagerError>;
    fn get_order(&self, id: u64);
    fn remove_order(&self, id: u64) -> Result<Vec<Order>, OrdersManagerError>;
}
*/
