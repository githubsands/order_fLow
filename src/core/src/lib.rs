// extern crate async_http_client;
use {
    curl::easy::Easy, std::collections::HashMap, std::io, std::io::stdout, std::io::Read,
    std::option, std::result, thiserror::Error,
};

pub struct Core {
    exchanges: vec![Exchange],
}

impl Core {
    pub fn new() {
        // exchanges = boot_exchanges()
        Self {
            // exchanges: exchanges,
        }
    }
    pub fn boot_exchanges() -> vec![Exchange] {}
    pub fn receive_signals() {
        loop {
            select! {
                recv(r) -> signal => println!("received signals"),
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Order {
    id: u64,
}

pub struct Config {
    orders_size: i64,
    id: i64,
    asset: String,
    style: String,
}

pub struct Exchanges {
    exchanges: HashMap<String, String>,
}

pub fn build_exchanges() -> Exchanges {
    let mut exchanges = HashMap::new();
    let key = String::from("local");
    let exchange = String::from("microexchange.svc.local");
    exchanges.insert(key, exchange);
    Exchanges {
        exchanges: exchanges,
    }
}

#[derive(Error, Debug)]
pub enum ExchangeError {
    #[error("exchange does not exist: {0}")]
    ExchangeDoesNotExist(String),
    #[error("orders do not exist: {0}")]
    ExchangeOrdersDoNotExit(string),
}

#[derive(Error, Debug)]
pub enum OrdersTraderErrors {
    #[error("order does not exist: {0}")]
    OrdersTraderOrderDoesExist(string),
    #[error("orders do not exist: {0}")]
    OrdersTraderOrdersDoNotExist(string),
}

impl Exchanges {
    pub fn get_exchange_uri(self, exchange_name: &str) -> Result<&str, ExchangeError> {
        let exchange = self.exchanges.get(exchange_name);
        match exchange {
            Some(_exchange) => Ok(exchange_name),
            None => {
                Err(ExchangeError::ExchangeDoesNotExist(exchange_name.to_string()))
            }
        }
    }
}

impl Trader {
    pub fn new(market_uri: &str, id: i64, orders_size: i8, asset: String, strategy: String) -> Self {
        let mut easy = Easy::new();
        easy.url(market_uri).unwrap();
        let orders: Vec<Order> = Vec::with_capacity(orders_size)
        Self {
            id: id,
            asset: asset,
            strategy: strategy,
            orders: orders,
            client: easy,
        }
    }
    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }
    pub fn orders(self) -> Result<Vec<Order>, TraderOrderNoErrorsExist>( {
        if self.orders.len() == 0 {
            Err(TraderClientError::TraderOrderNoOrdersExist())
        }
        Ok(self.orders);
    }
    pub fn orders_add(&mut self, mut orders: Vec<Order>) {
        self.orders.append(&mut orders)
    }
    pub fn get_order(&self, id: u64) -> Result<Order, TraderClientError> {
        for (_index, value) in self.orders.iter().enumerate() {
            if value.id == id {
                let order = value.clone();
                Ok(order);
            }
        }
        Err(TraderClientError::TraderOrderDoesNotExistError(id));
    }
    pub fn remove_order(&mut self, id: u64) -> Result<(), TraderClientError> {
        for order in &mut self.orders {
            if order.id == id {
                order.id = 0;
                Ok(());
            }
        }
        Err(TraderClientError::Trader:OrderDoesNotExistError(id));
    }
    // TODO: Use a better algorithm here then O(2n)
    pub fn remove_orders(&mut self, orders: Vec<i8>) -> Result<(), OrdersDoNotExistError> {
        for order in &mut self.orders {
            if order.id == id {
                self.orders (
            }
        }
    }
    pub fn send_order_to_exchange(&mut self, exchange: String) Result {
        handle.url(exchange).unwrap();
    }
}
