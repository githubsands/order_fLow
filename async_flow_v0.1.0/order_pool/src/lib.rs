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
