use std::fs;
use serde::Deserialize;
use reqwest::Client as WebClient;


#[derive(Deserialize, Debug)]
pub struct Source {
    id: u32,
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    sources: Vec<Source>,
}

#[tokio::main]
async fn main() {
    let contents = fs::read_to_string("config.toml")
        .expect("Something went wrong reading the file");

    // read config as Config struct from contents
    let config: Config = toml::from_str(&contents).unwrap();

    // print config
    println!("{:#?}", config);

    let web_client = WebClient::new();

    for source in config.sources {
        let response = web_client.get(source.url).send().await;
        println!("[{}] {}: {}", source.id, source.name, response.unwrap().status());
    }
}