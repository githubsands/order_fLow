# Order Flow - Algorithmic trader

Algorithimic order router server in rust using concurrent over async paradigms.

This instance hopes to leverage a set number of cores related to the number of exchanges
it trades with so performance isn't dampened by the CPU scheduler and
context switching

# Architecture and modules

. Exchange: defines websocket handling of each exchange participating and receives orders \
    from router. sends updated exchange parameters to core. \
. Strategy: the how how orders will be routed depending on signals generated from exchange parameters \
. Router: orders that have been strategically placed by strategy \
    from router. sends updated exchange parameters to core. receives routed orders from router. \
. Core: pulls the main modules together \

# Libraries used

(1) [ws-rs](https://github.com/housleyjk/ws-rs) - the only nonasync websocket client I could find. use [tokio mio](https://github.com/tokio-rs/miounder) under the hood \
(2) [cross-beam](https://github.com/crossbeam-rs/crossbeam) - a drop in crate for mspc \

# Inspired by

(1) https://www.amazon.com/Algorithmic-Trading-DMA-introduction-strategies/dp/0956399207 \
(2) https://academic.oup.com/book/2928 \
