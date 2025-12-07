use std::fs;

use toml::{Value, from_str};

fn main() {
    // load file as a string
    let raw = fs::read_to_string("./.env.toml")
    .expect("Cannot read ./.env.toml");
    // copy it inside a toml formatted value
    let config: Value = from_str(&raw)
    .expect("Invalid TOML");

    println!("{:#?}", config);
}
