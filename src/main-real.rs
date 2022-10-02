extern crate yaml_rust;

use yaml_rust::{YamlEmitter, YamlLoader};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    Exchanges: Vec<Exchanges>,
}

fn main() {
    cfg=YamlLoader::load_from_str(s).unwrap();
