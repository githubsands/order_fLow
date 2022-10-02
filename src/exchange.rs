extern crate ws;

use ws::listen;
use ws::{connect, CloseCode, Connection};

pub struct Exchange {
    name: &str,
    uri: &str,
    websocket: Connection,
    logger: env_logger,
}

#[derive(Error, Debug)]
pub enum ExchangeError {
    #[error("Error connecting to the exchange {}")]
    ExchangeErrorConnectingToExchange(string),
}

impl Exchange {
    pub fn new(name: &str, uri: &str) {
        Self {
            name: name,
            uri: uri,
        };
    }
    fn connect() -> Result<Error>{
        listen(self.uri, |out| {
            move |msg| {
                out.send(msg)
            }
        }
        })
    fn send(

}

fn start_exchange() {
    let exchangeURI = ""
    if let Err(error) = listen(exchangeURI, |out| {
        mov |msg| {
            println!("Server got message '{}'. ", msg);
            out.send(msg)
        }
    }){
        println!("failed to create connect to exchange {}", exchangeURI)
    }
}
