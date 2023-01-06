// extern crate async_http_client;
use crossbeam_channel::select;
use std::collections::HashMap;

extern crate Exchange;

use Exchange::*;

pub struct Exchanges {
    exchanges: HashMap<String, Exchange>,
    order_manager: OrderManager,
}

pub struct Core {
    exchanges: Exchanges,
}

impl Core {
    fn new() -> Self {
        let exchanges = HashMap::new();
        Self {
            // exchanges: exchanges,
        }
    }
}

impl Core {
    pub fn new(exchanges: Exchanges) -> Self {
        Self {
            exchanges: Exchanges,
        }
    }
    pub fn get_exchange_uri(self, exchange_name: &str) -> Result<&str, ExchangeError> {
        let exchange = self.exchanges.get(exchange_name);
        match exchange {
            Some(_exchange) => Ok(exchange_name),
            None => Err(ExchangeError::ExchangeDoesNotExist(
                exchange_name.to_string(),
            )),
        }
    }
    pub fn receive_signals() {
        loop {
            select! {
                recv(r) -> signal => println!("received signals"),
            }
        }
    }
}
