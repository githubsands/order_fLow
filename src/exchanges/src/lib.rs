// TODO: Move to exchanges lib

extern crate lazy_static;

use crate::exchange::Exchange;

use crossbeam_channel::select;
use crossbeam_channel::unbounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;

use std::collections::HashMap;

use std::sync::Arc;
use std::sync::RwLock;

lazy_static! {
    static ref HASHMAP: HashMap<&'static str, Arc<RwLock<Sender<i32>>>> = {
        let (exchange_one_sender, _) = unbounded();
        let (exchange_two_sender, _) = unbounded();
        let mut m = HashMap::new();
        m.insert("exchange_one", Arc::new(RwLock::new(exchange_one_sender)));
        m.insert("exchange_two", Arc::new(RwLock::new(exchange_two_sender)));
    };
}
