// use crate::signal::Signal;
use crossbeam_channel::{select, unbounded, Receiver, Sender};
use std::thread;

pub trait Strategy {
    fn allocate() {
        println!("allocated capital");
    }
    fn abandon() {
        println!("abandoned");
    }
    fn receive_signals() {
        println!("signals received");
    }
}

struct capital {
    location: &'static str,
    amount: u128,
}

// TODO: peculiar situation here.  how we grab 2 signals in the same time period
// to basis our decision from?
struct Signal {
    location: &'static str,
    price: u128,
    timestamp: u64,
}

struct Order {}

struct Arbritage {
    order_sender: Sender<Order>,
    signal_receiver: Receiver<Signal>,
}

impl Strategy for Arbritage {
    fn receive_signals() {
        let (s, r) = unbounded::<u8>(); // TODO: Change this type
        thread::spawn(move || loop {
            select! {
            recv(r) -> msg => println!("receiving signals")
            }
        });
    }
}
