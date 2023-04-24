use tokio::runtime::Runtime;

use std::io;
use serde::{Serialize, Deserialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use serde_json::Value;
use crossbeam_channel::{Receiver, Sender, unbounded};

use tokio::task;
use tokio::{pin, select};
use tokio::io::{split, ReadHalf, WriteHalf};
use futures::stream::Stream;

use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use std::collections::HashMap;


use tungstenite::{
    error::{Error, UrlError},
    handshake::client::Response,
    protocol::WebSocketConfig,
};

use url::{Url, ParseError};

lazy_static! {
    static ref exchanges: HashMap<&'static str, ExchangeController> = {
        let mut map = HashMap::new();
        map.insert("kraken", ExchangeController{uri: "ws://kraken.com".to_string()})
        map.insert("binance", ExchangeController{uri: "ws://binance.com".to_string()})
        map
    };
    static ref order_queue: Box::new(VecDeque::with_capacity(1000000));
}


#[derive(Serialize, Debug)]
struct Order {
    id: i16,
    item: String,
    quantity: f64,
}

#[derive(Deserialize, Debug)]
struct pricePoint {
    price_point: f64,
    amount: f64,
}

#[derive(Deserialize, Debug)]
struct OrderUpdate {
    offset: u64,
    asks: Vec<pricePoint>,
    bids: Vec<pricePoint>,
}

#[derive(Deserialize, Debug)]
struct OrderSnapShots {
    snap_shot: u8,
}

#[derive(Deserialize, Debug)]
struct OrderUpdates {
    offset: u64,
    orderbook_snapshot: Option<OrderSnapShots>,
    orders: Option<Vec<OrderUpdates>>,
}

pub struct Exchange< >{
    uri: &'static str,
    orderbook_message: Message,

    /// A `WebSocketStream<S>` represents a handshake that has been completed
    /// successfully and both the server and the client are ready for receiving
    /// and sending data. Message from a `WebSocketStream<S>` are accessible
    /// through the respective `Stream` and `Sink`. Check more information about
    /// them in `futures-rs` crate documentation or have a look on the examples
    ws_connection:  Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ws_write: Option<SplitStream>,
    ws_read: Option<SplitSink>,
    order_receiver: Receiver<Order>,
    tx: Sender<Order>,
}

impl Exchange {
    fn new(uri: &'static str, order_receiver: Receiver<Order>) -> (Exchange, Sender<Order>) {
        let (order_sender), mut order_receiver) = channel(1); // TODO: Tune channel here
        let orderbook_message = Message::Text("PER EXCHANGE ORDERBOOK MESSAGE GOES HERE".to_string());
        Exchange {
            uri: uri,
            ws_connection: None,
            order_receiver: order_receiver, /// Receives orders from upstream order placement
            order_update: order_update_sender, /// Sends OrderUpdates out to the order placer
            ws_write: None,
            ws_read: None,
        }
        (Exchange, order_sender)
    }
    async fn start(&mut self) -> Result<(), Error> {
        let (ws_connection, _) = connect_async(self.uri).await.expect("Failed to connection");
        let (reader, writer) = ws_connection.split();
        self.ws_read = Some(reader);
        self.ws_write = Some(writer);
        Ok(())
    }
    async fn read_websocket_connection(&mut self) {
        while let Some(msg) = self.ws_connection.unwrap().poll_next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    let order: Order = serde_json::from_str(&text).unwrap();
                    println!("Received order: {:?}", order);
                }
                Ok(Message::Close(_)) => {
                    // Handle WebSocket close message
                    break;
                }
                Err(e) => {
                    eprintln!("Error reading from WebSocket: {:?}", e);
                    break;
                }
                _ => {}
            }
        }
    }
    async fn write_websocket_connection(&mut self) {
        while let Some(order) = &self.rx.recv().await {
            &self.ws_write.unwrap().poll_write(order);
        }
    }
}


/// ExchangeControllers are leveraged by the order manager to 
/// send out orders and receive order updates from the respective
/// exchanges
struct ExchangeController {
    uri: &'static str,
    order_updates: Option<Receiver<OrderUpdates>>,
    order_senders: Option<Sender<Order>>,
}

struct OrderManager{
    num_exchanges: u64,
    exchange_io_executor: RunTime,
    exchanges: HashMap<String, ExchangeController>,
    order_pool: OrderPool
}

struct OrderPool {
    order_receiver:Receiver,
    orders: Box<VecDeque<OrderUpdate>>,
}

impl OrderPool {
    fn new() -> (OrderPool, Receiver) {
        let (order_update_sender, order_update_receiver) = unbounded();
        OrderPool {
            order_receiver: order_update_receiver,
            orders: orders,
        }
        (OrderPool, order_update_sender)
    }
    fn receive(&mut self) {
        let consumer = thread::spawn(move || {
            while let Ok(order_update) = self.order_receiver.recv() {
                self.orders.push_back(order_update);
            }
        });
    }
}

impl OrderManager {
    fn new() -> (OrderManager) {
       let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?;
       let op = OrderPool::new();
        OrderManager {
            num_exchanges: exchanges.len(),
            exchange_io_executor: rt,
            exchanges: exchanges,
            order_pool: op,
        }
    }
    /*
    fn fire_up_exchanges(&mut self) {
        while i < num_exchanges+1 {
            self.exchanges_io_executor.spawn(

                )
            }
        }
    */
}


impl Drop for Exchange {
    fn drop(&mut self) {
        let result = self.ws_connection.close(Some(CloseFrame));
    }
}
