# Order Flow - Algorithmic trader

Algorithimic order router server in rust using concurrent over async paradigms.

This instance hopes to leverage a set number of cores related to the number of exchanges
it tradds with signal processor so performance isn't dampened by the CPU scheduler and
conext switching

# Libraries used

(1) [ws-rs](https://github.com/housleyjk/ws-rs) - the only nonasync websocket client I could find
(3) [tokio mio](https://github.com/tokio-rs/miounder) under the hood
(2) [cross-beam](https://github.com/crossbeam-rs/crossbeam) - a drop in crate for mspc

# Inspired by

(1) https://www.amazon.com/Algorithmic-Trading-DMA-introduction-strategies/dp/0956399207
(2) https://academic.oup.com/book/2928
