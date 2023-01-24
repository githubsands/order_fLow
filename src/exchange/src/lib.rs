#![deny(warnings)]
#![allow(warnings, unused)]
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::{Arc, Mutex};
use ws::{connect, CloseCode, Factory, Handler, Message, WebSocket};

extern crate message;
extern crate order;
extern crate signal;
use message::Msg;

pub struct ExchangeConfig {
    exchange_uri: &'static str,
}

pub struct Exchange {
    name: &'static str,
    uri: &'static str,
    ws: Option<WebSocket>,
    chan_sender: Arc<Sender<Msg>>,
    chan_receiver: Arc<Receiver<Msg>>,
}

impl Exchange {
    pub fn new(name: &'static str, uri: &'static str) -> Exchange {
        let (s, r) = unbounded();
        Self {
            name: name,
            uri: uri,
            chan_sender: s,
            chan_receiver: r,
        }
    }
    pub fn start(&self) {
        let ws = connect(self.uri, |r| {
            move || loop {
                println!("starting")
            }
        });
        self.ws = ws;
    }
    /*
    pub fn execute_order(self) -> Result<(), ExchangeError> {}
    */
}

impl ws::Handler for Exchange {
    fn on_message(&mut self, msg: Message) -> Result<(), ()> {
        self.sender.send(msg);
    }
    //
    // fn on_open
    //
    // fn on_close
    //
    // fn on_error
    //
    //
}

impl Clone for Exchange {
    fn clone(&self) -> Exchange {
        Exchange {
            ws: self.ws.clone(),
            is_client: self.is_client.clone(),
            chan_sender: self.clone(),
            chan_receiver: self.clone(),
        }
    }
}

#[derive(Debug)]
pub enum ExchangeError {
    ExchangeErrorConnectingToExchange(String),
    ExchangeErrorSubmittingErrorToExchange(String, u8),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    #[test]
    fn test_ws_connection() {
        let host = Arc::new(Mutex::new("localhost:3082".to_string()));
        let cloned_host = Arc::clone(&host);
        let exchange_server = thread::spawn(move || {
            println!("running ws server");
            let host = cloned_host.lock()?;
            if let Err(error) = ws::listen(host.to_string(), |out| {
                move |msg| {
                    println!("Server got message '{}'", msg);
                    out.send(msg)
                }
            }) {
                println!("Failed to create websocket due to {:?}", error);
            }
        });
        let host_use = host.lock()?;
        let exchange_connection = Exchange::new(host_use.to_string());
        thread::sleep(time::Duration::from_millis(30));
    }
}
