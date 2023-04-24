pub trait Trade {
    fn send_order(&self, order: Order) -> Result<(), ()> {
        Ok(())
    }
    fn cancel_order(&self, order: Order) -> Result<(), ()> {
        Ok(())
    }
    fn new_order(&self, order: Order) -> Result<(), ()> {
        Ok(())
    }
}
