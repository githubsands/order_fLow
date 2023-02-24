#[derive(Debug)]
pub enum ExchangeError {
    ExchangeErrorConnectingToExchange(String),
    ExchangeErrorSubmittingErrorToExchange(String, u8),
}
