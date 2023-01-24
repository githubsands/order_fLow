// extern crate async_http_client;
use crossbeam_channel::select;
use crossbeam_channel::unbounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;

use threadpool::ThreadPool;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

extern crate exchange;
extern crate message;
extern crate router;
extern crate signal;
extern crate strategy;

use crate::exchange::Exchange;
use crate::message::Msg;
use crate::router::Router;
use crate::signal::Signal;
use crate::strategy::Arbritage;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct CoreConfig {
    strategy: &'static str,
}

pub struct Core {
    core_receiver: Arc<Receiver<Msg>>,
    core_sender: Arc<Sender<Msg>>,
    exchanges: ThreadPool,
    router: Router,
    router_thread: Option<std::thread>,
    strategizer: Arbritage,
    strategizer_thread: Option<std::thread>,
}

impl Core {
    fn new(config: config::Config}-> Self {
        let (sender, receiver) = unbounded();
        let exchange_thread_pool = ThreadPool::new(exchange_configs.len());
        let router = Router::new();
        let strategizer = Strategizer::new();
        Self {
            exchanges: exchange_thread_pool,
            core_receiver: Arc::new(receiver),
            core_sender: Arc::new(sender),
            router: router,
            strategizer: Arbritage,
        }
    }
    fn start_threads(&mut self) {
        self.router_thread = thread::spawn(move || self.router.route());
        self.strategizer_thread = thread::spawn(move || self.strategizer.receive_signals())
    }
    /*
    fn start_exchanges(&self, Vec<ExchangeConfig>) -> Result<(),error::Error> {
        &self.exchange_thread_pool.execute(|| {
            Exchange::new()
            // create new exchange instance
            // 1. pass in core's sender
            // 2. add it too exchange vector
        })
    }
    */
    /*
    pub fn receive_signals(&self) {
        loop {
            select! {
                recv(self.core_receiver) -> signal => println!("received signals"),
            }
        }
    }
    */
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
