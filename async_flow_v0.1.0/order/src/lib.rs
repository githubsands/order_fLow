/// OrderState keeps track of a order.
pub enum OrderState {
    Pending,
    Confirmed,
    Cancelled
}

/// Order defines a bare minimum order object to be send with an exchange.  This may vary
/// per exchange API
#[derive(Serialize, Debug)]
struct Order {
    id: i16,
    asset: String, // TODO: Do not pass around strings - possibly read from a global ASSET mapper
                   // and pass around an identifer for the ticket instead
    quantity: f64,
}

#[derive(Deserialize, Debug)]
struct PricePoint {
    price_point: f64,
    amount: f64,
}

#[derive(Deserialize, Debug)]
struct OrderBookUpdates {
    offset: u64,
    asks: Vec<PricePoint>,
    bids: Vec<PricePoint>,
}

#[derive(Deserialize, Debug)]
struct OrderSnapShots {
    snap_shot: u8,
}

#[derive(Deserialize, Debug)]
struct OrderUpdate {
    offset: u64,
    orderbook_snapshot: Option<OrderSnapShots>,
    orders: Option<Vec<OrderUpdate>>,
}

#[derive(Deserialize, Debug)]
struct OHLCUpdate {}

