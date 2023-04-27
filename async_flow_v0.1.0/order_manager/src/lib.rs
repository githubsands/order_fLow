

/// OrderPool holds a global orderbook from multiplie exchanges.  In later, future implementations this
/// should be a redis store rather then a VecDeque so we can store orderbooks across multiplie
/// instances
struct OrderPool {
    order_consumer: Receiver<OrderUpdate>,
    orders: Box<VecDeque<Order>>,
}

impl OrderPool {
    fn new() -> (OrderPool, Receiver<OrderUpdate>) {
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

// TODO: 
/*
/// FaultHandler handles faults from websocket clients and cordinates with the rest of the subsystems
/// appropriately depending on the strategy all Exchanges must be aligned and working
struct WSFaultHandler {

}
*/

/// OrderManager receives decisions from DecisionMaker on where to route orders
struct OrderManager {
    pending_orders: HashMap<&'static str, VecDeque<Order>>, /// Pending orders and where they are located
    order_pool: OrderPool
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
            order_state: Receiver<OrderState>,
        }
    }
    // TODO: Implement these functions below
    /*
    fn receive_order_state(&self) {
        while let Some(order) = &self.rx.recv().await {
            &self.ws_write.unwrap().poll_write(order);
        }
    }
    fn manage_pending_orders(&self) {

    }
    fn manage_submitted_orders(&self) {

    }
    */
}
