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

// Router receives signals and orders. to be sent to their respective exchanges components
pub struct Router {
    router_receiver: Arc<Receiver<Msg>>,
    router_sender: Arc<Sender<Msg>>,
    // exchanges: HashMap<&'static str, Arc<Mutex<Sender<Exchange>>>>,
}

impl Router {
    fn new() -> Self {
        let (sender, receiver) = unbounded();
        Self {
            router_receiver: Arc::new(receiver),
            router_sender: Arc::new(sender),
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
                                    // TODO: Handle errors
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
        // TODO:
        Ok(())
    }
    fn route_signal(self, signal: Signal) -> Result<(), RouterError> {
        // TODO::
        Ok(())
    }
    /*
    pub fn get_sender(&mut self) -> Arc<Sender<i32>> {
        return self.router_sender.lock().clone();
    }
    */
}
