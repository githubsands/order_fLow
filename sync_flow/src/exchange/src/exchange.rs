#![allow(warnings, unused)]

use crossbeam_channel::{select, unbounded, Receiver, Sender};

use std::sync::{Arc, Mutex};
use ws::util::Token;
use ws::{
    connect, CloseCode, Error, Factory, Handler, Handshake, Message as WSMessage, Request,
    Response, WebSocket,
};

extern crate message;
extern crate order;
extern crate signal;

use message::Msg;
use order::Order;

pub struct Exchange {
    name: &'static str,
    websocket: Option<Box<ws::Sender>>,
    uri: &'static str,
    chan_sender: Arc<Sender<Msg>>,
    order_receiver: Arc<Receiver<Msg>>,
    is_client: bool,
}

impl Exchange {
    pub fn new(name: &'static str, uri: &'static str) -> Exchange {
        let (sender, receiver) = unbounded();
        Self {
            name: name,
            websocket: None,
            uri: uri,
            is_client: true,
            order_receiver: Arc::new(receiver),
            chan_sender: Arc::new(sender),
        }
    }
    pub fn receiver(self) -> Arc<Receiver<Msg>> {
        return self.order_receiver.clone();
    }
    fn receive_orders(self) {
        let websocket = self.websocket.unwrap();
        select! {
        recv(*self.order_receiver) -> order =>
            {
                websocket.send(ws::Message::Text("test".to_string()));
            }
        }
    }
    fn set_ws(&mut self, websocket: ws::Sender) {
        self.websocket = Option::Some(Box::new(websocket))
    }
}

impl Handler for Exchange {
    fn on_shutdown(&mut self) {}
    fn on_open(&mut self, shake: Handshake) -> Result<(), ws::Error> {
        Ok(())
    }
    fn on_message(&mut self, msg: WSMessage) -> Result<(), ws::Error> {
        self.chan_sender.send(Msg::from_ws_message(msg));
        Ok(())
    }
    fn on_close(&mut self, code: CloseCode, reason: &str) {}
    fn on_error(&mut self, err: Error) {}
}

impl Factory for Exchange {
    type Handler = Exchange;
    fn connection_made(&mut self, ws: ws::Sender) -> Exchange {
        let mut exchange = Exchange::new("test", "test");
        exchange.set_ws(ws);
        exchange
    }
    fn on_shutdown(&mut self) {}
}

impl Clone for Exchange {
    fn clone(&self) -> Exchange {
        Exchange {
            name: self.name.clone(),
            uri: self.uri.clone(),
            websocket: self.websocket.clone(),
            is_client: false,
            chan_sender: self.chan_sender.clone(),
            order_receiver: self.order_receiver.clone(),
        }
    }
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
            println!("Running WS server");
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
    #[test]
    fn test_factory() {
        let exchange_server = thread::spawn(move || {
            println!("Running WS server");
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
        exchange_factory = ExchangeFactory::new("test_server", host.to_string());
    }
}
