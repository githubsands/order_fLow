#![deny(warnings)]

crate::config::*,
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use toml;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    exchanges: Option<HashMap<&'static str, ExchangeConfig>>,
}

pub fn deserialize_config(config_file_name: &str) -> Config {
    let mut process_config_filepath = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    process_config_filepath.push_str(config_file_name);
    println!("file path is {}", process_config_filepath);
    let config_file = File::open(process_config_filepath).unwrap();
    let buf_reader = BufReader::new(config_file);
    let config_bytes = buf_reader.buffer();
    let config: Config = toml::from_slice(&config_bytes).unwrap();
    config
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deserialize_config() {
        let mut exchanges = HashMap::new();
        exchange_config1 = ExchangeConfig {
            exchange_uri: "test_1_uri",
        };
        exchange_config2 = ExchangeConfig {
            exchange_uri: "test_2_uri",
        };
        exchanges.insert("test_1", exchange_config1);
        exchanges.insert("test_2", exchange_config2);
        let expected = Config {
            exchanges: Some(exchanges),
        };
        let actual = deserialize_config("config.toml");
        assert_eq!(actual, expected)
    }
}
