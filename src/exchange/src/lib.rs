#![allow(warnings, unused)]

mod exchange;

use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::{Arc, Mutex};
use ws::{connect, CloseCode, Factory, Handler, Message, WebSocket};

extern crate message;
extern crate order;
extern crate signal;
use message::Msg;
use order::Order;
