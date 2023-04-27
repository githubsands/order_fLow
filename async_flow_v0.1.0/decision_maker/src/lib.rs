
/// TODO: Implement DecisionMaker
/// DecisionMaker is where strategy is execution occurs dependent our information signals.
/// It receives OHLC information. Decision from this information and tells OrderManager where to route orders
struct DecisionMaker {
    ohlc_receiver: Receiver<OHLCUpdate>,
}
