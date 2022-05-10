// extern crate async_http_client;
use {
    curl::easy::Easy, std::collections::HashMap, std::io, std::io::stdout, std::io::Read,
    std::option, std::result, thiserror::Error,
};

// use async_http_client::{HttpCodec, HttpRequest};
//
fn main() {}

#[derive(Clone, Debug, PartialEq)]
pub struct Order {
    id: u64,
}

pub struct Config {
    id: i64,
    asset: String,
    style: String,
}

#[allow(unused_variables)]
pub struct Trader {
    id: i64,
    asset: String,
    strategy: String,

    orders: Vec<Order>,

    client: Easy,
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
}

impl Exchanges {
    pub fn get_exchange_uri(self, exchange_name: &str) -> Result<&str, ExchangeError> {
        let exchange = self.exchanges.get(exchange_name);
        match exchange {
            Some(_exchange) => Ok(exchange_name),
            None => {
                return Err(ExchangeError::ExchangeDoesNotExist(
                    exchange_name.to_string(),
                ))
            }
        }
    }
}

//    let mut map = HashMap::new();
#[derive(Error, Debug)]
pub enum TraderClientError {
    #[error("order does not exist: {0}")]
    TraderOrderDoesNotExistError(u64),
}

impl Trader {
    pub fn new(market_uri: &str, id: i64, asset: String, strategy: String) -> Self {
        let mut easy = Easy::new();
        easy.url(market_uri).unwrap();

        let orders: Vec<Order> = Vec::new();
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
    pub fn orders(self) -> Vec<Order> {
        return self.orders;
    }
    pub fn orders_add(&mut self, mut orders: Vec<Order>) {
        self.orders.append(&mut orders)
    }
    pub fn get_order(&self, id: u64) -> Result<Order, TraderClientError> {
        for (_index, value) in self.orders.iter().enumerate() {
            if value.id == id {
                let order = value.clone();
                return Ok(order);
            }
        }
        return Err(TraderClientError::TraderOrderDoesNotExistError(id));
    }
    pub fn remove_order(&mut self, id: u64) -> Result<(), TraderClientError> {
        for order in &mut self.orders {
            if order.id == id {
                order.id = 0;
                return Ok(());
            }
        }
        return Err(TraderClientError::TraderOrderDoesNotExistError(id));
    }
    /*
    pub fn send_order_to_exchange(&mut self, exchange: String) Result {
        handle.url(exchange).unwrap();
    }
    */
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trader_client() {
        let id = i64::from(12);
        let asset = String::from("BTC/USD");
        let strategy = String::from("market-maker");
        let trader_client = build_trader(12, asset, strategy);
        let order = Order { id: 12 };
        trader_client.add_order(order);
    }
}
