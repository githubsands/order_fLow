use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::{Message, CloseFrame};
use futures::stream::{SplitStream, SplitSink};

use tokio::runtime::{Builder, Runtime};
use tokio::net::TcpStream;
use tokio::io::{split, ReadHalf, WriteHalf};
use futures::stream::Stream;

use crossbeam_channel::{Receiver, Sender, unbounded};

use url::{Url, ParseError};
use serde::{Serialize, Deserialize};

use std::io;
use std::collections::{HashMap, VecDeque};

#[derive(Deserialize, Debug)]
struct Config {
    volume_limit_max: u32,
    volume_limit_min: u32,
    max_order_size: u8,
    min_order_size: u8,
}

/// ATSError holds all possibly errors for the ATS system so no dynamic allocation (dyn Error) occur
/// in the hot path.  Errors are divided to infrastructure layer errors and application layer
/// errors due to Implementation Shortfalls (IS)
#[derive(Debug)]
enum ATSError {
    WebSocketServerDisconnect,
    OrderDidNotFill,
}

pub struct ATs {
    exchangeIO: ExchangeIO,
    decisionMaker: DecisionMaker,
    order_manager: OrderManager,
}
