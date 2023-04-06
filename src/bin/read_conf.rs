use std::fs;
use toml::from_str;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Source {
    id: u32,
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    sources: Vec<Source>,
}

fn main() {
    let contents = fs::read_to_string("config.toml")
        .expect("Something went wrong reading the file");

    // read config as Config struct from contents
    let config: Config = from_str(&contents).unwrap();

    // print config
    println!("{:#?}", config);
}