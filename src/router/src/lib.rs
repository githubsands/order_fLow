#[macro_use]
extern crate lazy_static;

use crossbeam_channel::select;
use crossbeam_channel::unbounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;

use std::collections::HashMap;

use std::sync::Arc;
use std::sync::RwLock;

// TODO: Move to exchanges lib
lazy_static! {
    static ref HASHMAP: HashMap<&'static str, Arc<RwLock<Sender<i32>>>> = {
        let (exchange_one_sender, _) = unbounded();
        let (exchange_two_sender, _) = unbounded();
        let mut m = HashMap::new();
        m.insert("exchange_one", Arc::new(RwLock::new(exchange_one_sender)));
        m.insert("exchange_two", Arc::new(RwLock::new(exchange_two_sender)));
        m
    };
}

// Router receives orders upstream to be sent to their respective exchanges
pub struct OrderRouter {
    router_receiver: Arc<RwLock<Receiver<i32>>>, // TODO: change type to OrderRouter channels
    router_sender: Arc<Sender<i32>>,
    // exchanges: HashMap<&'static str, Arc<RwLock<Sender<i32>>>>,
}

impl OrderRouter {
    fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self {
            router_receiver: Arc::new(RwLock::new(receiver)),
            router_sender: Arc::new(sender),
        }
    }
    fn route_orders(self) {
        loop {
            select! {
                recv(*self.router_receiver.clone().write().unwrap()) -> order => println!("received order to be routed"),
            }
        }
    }
    pub fn get_sender(&mut self) -> Arc<Sender<i32>> {
        return self.router_sender.clone();
    }
}
