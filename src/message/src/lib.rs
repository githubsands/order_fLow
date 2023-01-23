extern crate order;
extern crate signal;

use message_handler::FromWSMsg;
use order::Order;
use signal::Signal;

pub enum Msg {
    Signal(Signal),
    Order(Order),
}

enum MsgResultError {
    WrongType(String),
}

impl Msg {
    fn signal<T>(&self) -> Result<&Signal, MsgResultError>
    where
        T: FromWSMsg,
    {
        match &*self {
            Msg::Signal(x) => Ok(x),
            Msg::Order(_) => Err(MsgResultError::WrongType("Order".to_string())),
        }
    }

    fn order<T>(&self) -> Result<&Order, MsgResultError>
    where
        T: FromWSMsg,
    {
        match &*self {
            Msg::Order(x) => Ok(x),
            Msg::Signal(_) => Err(MsgResultError::WrongType("Trade".to_string())),
        }
    }
}
