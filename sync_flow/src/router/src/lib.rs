#[macro_use]
extern crate lazy_static;
extern crate message;
extern crate order;
extern crate signal;
use message::Msg;
use order::Order;
use signal::Signal;

use crossbeam_channel::select;
use crossbeam_channel::unbounded;
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;

use std::collections::HashMap;

use std::sync::Arc;
use std::sync::Mutex;

enum RouterError {
    FailedToRouteOrder,
    FailedToRouteSignal,
}

pub struct Config {
    thread_stacksize: Option<&'static str>,
}

// Router receives signals and orders. to be sent to their respective exchanges components
pub struct Router {
    router_receiver: Arc<Receiver<Msg>>,
    router_sender: Arc<Sender<Msg>>,
    exchange_senders: Arc<HashMap<&'static str, Sender<Msg>>>,
}

impl Router {
    pub fn new(exchange_senders: Arc<HashMap<&'static str, Sender<Msg>>>) -> Self {
        let (sender, receiver) = unbounded();
        Self {
            router_receiver: Arc::new(receiver),
            signal_receiver: signal_receiver,
            exchange_senders: exchange_senders,
        }
    }
    pub fn route(self) {
        loop {
            select! {
                    recv(*self.router_receiver) -> msg_crossbeam =>
                    {
                        let msg = msg_crossbeam.unwrap();
                        match msg {
                            Msg::Order(order) => {
                                let result = match self.route_order(order) {
                                    Ok(_) | Err(_) => todo!(),
                                };
                            }
                            Msg::Signal(signal) => {
                                let result = match self.route_signal(signal) {
                                    // TODO: Handle errors
                                    Ok(_) | Err(_) => todo!(),
                                };
                        }
                    }
                }
            }
        }
    }
    fn route_order(self, order: Order) -> Result<(), RouterError> {
        let senders = self.exchange_senders.get(order.exchange);
        match senders {
            Some(senders) => {
                senders.send(Msg::Order(order));
                Ok(())
            }
            None => Err(RouterError::FailedToRouteOrder),
        }
    }
    fn route_signal(self, signal: Signal) -> Result<(), RouterError> {
        self.signal_sender(Msg::Sender(signal));
    }
}
/*
pub fn get_sender(&mut self) -> Arc<Sender<i32>> {
    return self.router_sender.lock().clone();
}
*/
