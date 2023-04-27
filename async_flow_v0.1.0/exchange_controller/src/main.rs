
struct ExchangeIO {
    num_exchanges: u64,
    exchange_io_executor: RunTime,
    exchanges: HashMap<String, ExchangeController>,
}

/// ExchangeControllers are leveraged by the order manager to
/// send out orders, receive order and ohlc updates, and keep
/// aware of exchange failures from exchanges
struct ExchangeController {
    uri: &'static str,
    order_book_updates: Option<Receiver<OrderBookUpdate>>, ///:
    order_executor: Option<Sender<Order>>,
    halted_signal: Option<Receiver<bool>>,
    halted: bool,
}

impl ExchangeController {
    fn new(exchange_api: &ExchangeAPI, order_book_updates: Receiver<OrderUpdate>, order_executor: Sender<Order>) -> ExchangeController {
        ExchangeController {
            uri:  exchange_api.ws_uri,
            order_book_updates: order_book_updates, /// Sends orders to the exchange.
            order_executor: order_executor, /// Receive OrderUpdates from the exchange  - updates.
        };
        ExchangeController
    }
    async fn start_halt_loop(&self) {
        while let Some(order) = &self.halted_signal.recv().await {
            &self.ws_write.unwrap().poll_write(order);
        }
    }
}

// TODO: 
/*
/// FaultHandler handles faults from websocket clients and cordinates with the rest of the subsystems
/// appropriately depending on the strategy all Exchanges must be aligned and working
struct WSFaultHandler {

}
*/
