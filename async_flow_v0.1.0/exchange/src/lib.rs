
//! A simple example of hooking up stdin/stdout to a WebSocket stream.
//!
//! This example will connect to a server specified in the argument list and
//! then forward all data read on stdin to the server, printing out all data
//! received on stdout.
//!
//! Note that this is not currently optimized for performance, especially around
//! buffer management. Rather it's intended to show an example of working with a
//! client.
//!
//! You can use this example together with the `server` example.

use std::env;
use std::writeln;
use std::fmt::*;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::{future, pin_mut, StreamExt};
use futures::stream::{SplitStream, SplitSink};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream, MaybeTlsStream};
use async_trait::async_trait;
use std::collections::VecDeque;

use futures::{Stream};
use async_stream::stream;

use std::rc::Rc;
use std::cell::RefCell;

use tokio::net::TcpStream;

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, Sender, Receiver, UnboundedSender};

lazy_static::lazy_static! {
    static ref exchange_apis: HashMap<&'static str, ExchangeAPI>= HashMap::new();
        exchange_apis.insert("tbd", exchangeAPI{ws_uri: "tbd", orderbook_subscription: "orderbook/$asset", ohlc_subscription: "ohlc/$asset"});
        exchange_apis.insert("tbd", exchangeAPI{ws_uri: "tbd", orderbook_subscription: "orderbook/$asset", ohlc_subscription: "ohlc/$asset"});
    static ref ORDER_QUEUE: Box::new(VecDeque::with_capacity(1000000));
}

/// ExchangeAPI holds all messages we use to interact with a exchange
pub struct ExchangeAPI {
    ws_uri: &'static str,
    orderbook_subscription: &'static str,
    ohlc_subscription: &'static str,
}

/// Exchange implements an interface over a external exchange that we trade with.
/// It keeps track of the pairs we are trading and each websocket request we want
/// to trade with. For our simple strategy we currently have three 3 websockets per pair
/// and maintain a subscription per websocket rather then have multiplie websockets per
/// subscription
pub struct Exchange {
    /// URI is where this exchange is located
    uri: &'static str,

    /// Subscription  messages that are required to get data feeds from exchanges.
    /// necessary on a websocket startup as well as disconnect.
    orderbook_subscription_message: Message,
    ohlc_subscription_message: Message,

    /// The pairs to keep track of. For now we will default these pairs at size one
    /// for our basic strategy. 
    watched_pairs: [&'static str; 2],

    order_executor_consumer: Receiver<Order>, // order_executor_consumer is a part of a single consumer and producer
                             // channel

    /// A `WebSocketStream<S>` represents a handshake that has been completed
    /// successfully and both the server and the client are ready for receiving
    /// and sending data. Message from a `WebSocketStream<S>` are accessible
    /// through the respective `Stream` and `Sink`. Check more information about
    /// them in `futures-rs` crate documentation or have a look on the examples
    ws_connection_orderbook:  Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ws_connection_ohlc:  Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ws_connection_order_executors:  Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,

    ws_write_orderbook: Option<SplitStream<Message>>,
    ws_read_orderbook: Option<ReadHalf>,
    order_running: bool,

    ws_write_ohlc: Option<SplitStream<Message>>,
    ws_read_ohlc: Option<ReadHalf>,
    ohlc_running: bool,

    ws_write_order_executor: Option<SplitStream<Message>>,
    ws_read_order_executor: Option<ReadHalf>,
    order_executor_running: bool,
}

impl Exchange {
    fn new(exchange_api: &ExchangeAPI, order_executor_consumer: Receiver<Order>) -> (Exchange, Sender<Order>) {
        let (order_executor_producer, mut order_executor_consumer) = unbounded();
        Exchange {
            uri: exchange_api.uri,
            orderbook_subscription_message: Message::Text(exchange_api.orderbook_subscription.to_string()),
            ohlc_subscription_message: Message::Text(exchange_api.ohlc_subscription.to_string()),
            watched_pairs: ["ETH/USD", "BTC/USD"], /// Watched Pairs are hard coded for now
            order_executor_consumer: order_executor_consumer,
            ws_connection_orderbook: None,
            ws_connection_ohlc: None,
            ws_connection_order_executors:  None,
            ws_write_orderbook: None,
            ws_read_orderbook: None,
            order_running: false,
            ws_write_ohlc: None,
            ws_read_ohlc: None,
            ohlc_running: false,
            ws_write_order_executor: None,
            ws_read_order_executor: None,
            order_executor_running: false,
        };
        (Exchange, order_executor_producer)
    }
    async fn start_orderbook_connection(&mut self) -> Result<(), ATSError> {
        let (ws_connection_orderbook, _) = connect_async(self.uri).await.expect("Failed to connection");
        let (reader, writer) = ws_connection_orderbook.split();
        self.ws_write_orderbook = Some(writer);
        self.ws_read_orderbook = Some(reader);
        self.ohlc_orderbook = true;
        Ok(())
    }
    async fn start_ohlc_connection(&mut self) -> Result<(), ATSError> {
        let (ws_connection_orderbook, _) = connect_async(self.uri).await.expect("Failed to connection");
        let (reader, writer) = ws_connection_orderbook.split();
        self.ws_write_ohlc = Some(writer);
        self.ws_read_orderbook = Some(reader);
        self.ohlc_running = true;
        Ok(())
    }
    /*
    // TODO: Remove this sense we are using Streams
    async fn read_websocket_connection(&mut self) {
        while let Some(msg) = self.ws_connection.unwrap().poll_next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    let order: Order = serde_json::from_str(&text).unwrap();
                    println!("Received order: {:?}", order);
                }
                // TODO:                 // On receiving a closed websocket request we need to handle it appropriately
                // depending on the websocket close message, the exchanges websocket service bias
                // and our overal strategy for this deployment.
                //
                // See https://www.rfc-editor.org/rfc/rfc6455#section-7.4 for more websocket details.
                // Be erie on producing blocked code here - async functions must return quickly
                Ok(Message::Close(close_code)) => {
                    let resolved = false;
                    match close_code {
                        101 => println!("do work"),
                        102 => println!("do work"),
                        _ => println!("undefined status code - do work")
                    }
                }
                // Handle websocket request in a similar manner to Close here.
                Err(e) => {
                    eprintln!("Error reading from WebSocket: {:?}", e);
                    continue;
                }
                // Handle websocket request here in a similar manner to Close and Err.
                _ => {}
            }
        }
    }
    */
    /* TODO: Remove this sense we are using streams
    async fn write_websocket_connection(&mut self) {
        while let Some(order) = &self.rx.recv().await {
            &self.ws_write.unwrap().poll_write(order);
        }
    }
    */
}

impl Drop for Exchange {
    fn drop(&mut self) {
        let result = self.ws_connection.close(Some(CloseFrame));
    }
}

/// Stream yields nothing but forwards our messages to the order manager.  It is a producer.
/// read_guy has two states:
///
/// (1) buffer has items
/// (2) buffer does not have items
///
/// if the buffer has items we return Poll::Ready(()). this then sends items to our order_manager
/// if the buffer has no items we check our underlying stream and return and load our buffer
///
/// TODO: We may possibly be able to move to https://crates.io/crates/async-stream here
impl Stream for Exchange {
    type Item = ();

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<()>> {
        // Lets first check if our buffer has any messages ready.  If messages are ready
        // send them to our order manager
        if let Some(msg) = &mut self.buffer.pop_front() {
            self.order_producer.send(*msg);
            Poll::Ready(Some(()));
        }
        // Pin ws_incoming_stream on the stack and check for reading messages.
        // if messages are ready at them to the queue. We implementing futures we only
        // pin inner futures
        let mut pinned_stream = Pin::new(&mut self.ws_incoming_stream);

        let order = pinned_stream.poll_next(cx);
        match order {
            Poll::Ready(Some(Ok(_))) => {
                let msg: u8 = 12;
                &mut self.buffer.push_back(msg);
                Poll::Ready(Some(()))
            }
            // We received a downstream error - don't crash the program but resolve it using other
            // means.  Depending on our strategy one websocket being momentarily down
            Poll::Ready(Some(Err(e))) => {
                Poll::Pending
            }
            Poll::Ready(None) => {
                Poll::Pending
            }
            Poll::Pending =>  {
                Poll::Pending
            }
        }
    }
}
