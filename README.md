# <span style="color:green">Order Flow - Algorithmic trader</span>

Order flow is divided into two servers - sync an async.  Each server
leverages different paradigms concurrency through os threads or epolling, green
threads (tasks), and concurrency.

Both servers are a WIP. The former sync only server is an abandoned project.

## <span style="color:red">sync</span>

Algorithimic order router server in rust using concurrent over async paradigms.

This instance hopes to leverage a set number of cores related to the number of exchanges
it trades with so performance isn't dampened by the CPU scheduler and
context switching

The process runs threads for the following flows:

* strategizer : needed to set strategy on capital given exchange or other signal inputs
* router: directs signal and order objects to their proper locations: strategizer and exchange
* exchange(s): runs a loop on each websocket connection
* 
### Architecture and modules:

. Exchange: defines websocket handling of each exchange participating and receives orders \
    from router. sends updated exchange parameters to core. \
. Strategy: the how how orders will be routed depending on signals generated from exchange parameters \
. Router: orders that have been strategically placed by strategy \
    from router. sends updated exchange parameters to core. receives routed orders from router. \
. Core: pulls the main modules together \

### Libraries used

(1) [ws-rs](https://github.com/housleyjk/ws-rs) - the only nonasync websocket client I could find. use [tokio mio](https://github.com/tokio-rs/miounder) under the hood \
(2) [cross-beam](https://github.com/crossbeam-rs/crossbeam) - a drop in crate for mspc \

## <span style="color:red">async</span>

Async for websocket client I/O and sync for internal CPU related work.

### Architecture and modules:

I/O related workloads are ran on a tokio executor. Tasks (websocket reads)
are sent to the sync portion through a crossbeam producer to be handled.

#### modules TBD

### Libraries used

(1) tungesenite - websocket server

(2) tokio - async related libraries

(3) future-utils - more async tools

(4) crossbeam - mpsc channel

### Inspired by

(1) https://www.amazon.com/Algorithmic-Trading-DMA-introduction-strategies/dp/0956399207 \
(2) https://academic.oup.com/book/2928 \
