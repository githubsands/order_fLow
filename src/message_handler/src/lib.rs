pub enum WSMsgError {
    Empty,
}

pub trait FromWSMsg {
    fn is_from_ws(&self) {
        println!("is from self")
    }
    fn is_empty(&self) -> Result<(), WSMsgError> {
        Err(WSMsgError::Empty)
    }
}
