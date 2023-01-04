extern crate ws;

// kuse std::error::Error;
use crossbeam_channel::{unbounded, Sender};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use ws::{connect, listen, CloseCode, Factory, Handler, WebSocket};

pub struct Exchange {
    ws: WebSocket<FnMut<ws::Sender>>,
    is_client: bool,
    chan_out: Sender<String>,
    chan_in: Sender<String>,
}

impl Exchange {
    pub fn new(exchange_uri: String) -> Result<Exchange, ExchangeError> {
        let (s, r) = unbounded::<String>();
        let ws = connect(exchange_uri, |r| {
            move |msg| {
                println!("Got message: {}", msg);
                r.close(CloseCode::Normal)
            }
        })
        .unwrap();
        Self {
            ws: ws,
            chan_out: s,
            chan_in: r,
        }
    }
}

impl Clone for Exchange {
    fn clone(&self) -> Exchange {
        Exchange {
            ws: self.ws.clone(),
            is_client: self.is_client.clone(),
        }
    }
}

/* TODO: Implement Drop
impl Drop for Exchange {
    fn drop(&mut self) {
        self.ws.
    }
}
*/

#[derive(Debug)]
pub enum ExchangeError {
    // #[error("Error connecting to the exchange {}")]
    ExchangeErrorConnectingToExchange(String),
}

/*
impl Handler for Exchange {}

struct ExchangeFactory;

impl Factory for ExchangeFactory {
    type Handler = Exchange;
    fn client_connected(&mut self, ws: Sender) -> Exchange {
        Exchange {
            ws: ws,
            is_client: true,
        }
    }
    fn connection_made(&mut self, ws: Sender) -> Exchange {
        Exchange {
            ws: ws,
            is_client: false,
        }
    }
}
*/

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
            let host = cloned_host.lock().unwrap();
            if let Err(error) = ws::listen(host.to_string(), |out| {
                move |msg| {
                    println!("Server got mesage '{}'", msg);
                    out.send(msg)
                }
            }) {
                println!("Failed to create websocket due to {:?}", error);
            }
        });
        let host_use = host.lock().unwrap();
        let exchange_connection = Exchange::new(host_use.to_string());
        thread::sleep(time::Duration::from_millis(30));
    }
}
