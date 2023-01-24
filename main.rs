use std::env;
use std::error::Error;
extern crate config;
extern crate core;
use config::{Config, Deserialize_config};
use core::Core;

fn main() {
    let result = order_flow();
    match result {
        Ok(_) => println!("successful shutdown"),
        Err(e) => println!("failed to run exchange {}", e),
    }
}

fn order_flow() -> Result<(), error::Error> {
    match env::var("CONFIG_PATH") {
        Ok(result) => {
            let cfg = Deserialize_config();
            let _core = core::new(cfg);
            Ok(())
        }
        Err(e) => Err(e),
    }
}
