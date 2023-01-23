use std::error::Error;
use std::iter::Iterator;
use std::vec;

extern crate message_handler;
use message_handler::FromWSMsg;

#[derive(Clone, Debug)]
enum OrderType {
    LimitOrder,
    StopLossOrder,
    TrailingOrder,
}

#[derive(Clone, Debug)]
pub struct Order {
    kind: OrderType,
    id: i64,
    capital: u8,
}

impl FromWSMsg for Order {}

pub enum OrdersManagerError {
    OrdersNonExistent,
    OrderDoesntExist(i64),
}

#[derive(Clone, Debug)]
pub struct OrderManager {
    orders: Vec<Order>,
    count: usize,
}

impl OrderManager {
    fn new() -> Self {
        Self {
            orders: vec![],
            count: 0,
        }
    }
    fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }
    fn orders(self) -> Result<Vec<Order>, OrdersManagerError> {
        if self.orders.len() == 0 {
            Err(OrdersManagerError::OrdersNonExistent)?
        }
        Ok(self.orders)
    }
    fn orders_add(&mut self, mut orders: Vec<Order>) {
        self.orders.append(&mut orders)
    }
    fn get_order(&self, id: i64) -> Result<Order, OrdersManagerError> {
        for (_index, value) in self.orders.iter().enumerate() {
            if value.id == id {
                let order = value.clone();
                return Ok(order);
            }
        }
        Err(OrdersManagerError::OrderDoesntExist(id))?
    }
    fn remove_order(&mut self, id: i64) -> Result<i64, OrdersManagerError> {
        for order in &mut self.orders {
            if order.id == id {
                order.id = 0;
                return Ok(id);
            }
        }
        Err(OrdersManagerError::OrderDoesntExist(id))?
    }
}

impl Iterator for OrderManager {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < self.orders.len() {
            Some(self.count)
        } else {
            None
        }
    }
}
