use std::fs;
use reqwest::blocking::Client;
use toml::Value;

fn main() {
    let contents = fs::read_to_string("config.toml")
        .expect("Something went wrong reading the file");

    let config: Value = contents.parse().unwrap();

    let client = Client::new();

    for (source, endpoint) in config["sources"].as_table().unwrap() {
        let url = endpoint.as_str().unwrap();
        let response = client.get(url).send().unwrap();
        println!("{}: {:?}", source, response);
    }
}