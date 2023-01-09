// extern crate async_http_client;
use crossbeam_channel::select;
use crossbeam_channel::unbounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;

use std::collections::HashMap;

use std::sync::Arc;

pub struct Core {
    core_receiver: Receiver<i32>,
    core_sender: Arc<Sender<i32>>,
}

impl Core {
    fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self {
            core_receiver: receiver,
            core_sender: Arc::new(sender),
        }
    }
}

impl Core {
    /*
    pub fn get_exchange_uri(self, exchange_name: &str) -> Result<&str, ExchangeError> {
        let exchange = self.exchanges.get(exchange_name);
        match exchange {
            Some(_exchange) => Ok(exchange_name),
            None => Err(ExchangeError::ExchangeDoesNotExist(
                exchange_name.to_string(),
            )),
        }
    }
    */
    pub fn receive_signals(&self) {
        loop {
            select! {
                recv(self.core_receiver) -> signal => println!("received signals"),
            }
        }
    }
    pub fn get_sender(&mut self) -> Arc<Sender<i32>> {
        return self.core_sender.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    #[test]
    fn test_receive_signals() {
        let mut core = Core::new();
        let sender = core.get_sender();
        let _core_thread = thread::spawn(move || {
            println!("receiving signals");
            core.receive_signals();
        });
        println!("sending signals");
        sender.send(12);
        thread::sleep(time::Duration::from_millis(300000));
    }
}
