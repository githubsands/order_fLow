use crossbeam_channel::{select, unbounded, Receiver, Sender};
use std::thread;

pub struct StrategyConfig {
    kline_period: &'static str,
}

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

pub struct Arbritage {
    asset_one: &'static str,
    asset_tow: &'static str,
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
