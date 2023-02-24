use std::collections::HashMap;
extern crate message_handler;
use message_handler::FromWSMsg;

// Signal struct:
//    holds parameter updates on a specific market. orderbooks
//    is represent as a tuple(amounts,price_point)
pub struct Signal {
    // TODO: Implement this here Signal --
    /*
    volume: usize,
    price: i32,
    exchange: String,
    kline: kline,
    */
}

impl Signal {
    pub fn new() -> Self {
        Self {}
    }
}

impl FromWSMsg for Signal {}

struct kline {
    open_time: i64,
    close_time: i64,
    open_price_high: i32,
    open_price_time: i32,
    high_price: i32,
    low_price: i32,
    volume: i8,
    trades: i8,
    taker_buy: i64,
    taker_buy_quote: i64,
    rate_limit: rateLimit,
}

enum rateLimitType {
    REQUEST_WEIGHT,
}

enum interval {
    MINUTE,
}

struct rateLimit {
    rate_limit_type: rateLimitType,
    kind: &'static str,
    limit: interval,
}
