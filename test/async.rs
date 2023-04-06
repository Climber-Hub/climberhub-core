use std::fs;
use reqwest::Client;
use tokio::runtime::Runtime;
use toml::Value;

fn main() {
    let contents = fs::read_to_string("config.toml")
        .expect("Something went wrong reading the file");

    let config: Value = contents.parse().unwrap();

    let mut rt = Runtime::new().unwrap();
    let client = Client::new();

    rt.block_on(async {
        let futures = config["sources"]
            .as_table()
            .unwrap()
            .iter()
            .map(|(source, endpoint)| {
                let url = endpoint.as_str().unwrap();
                client.get(url).send()
            });

        for (source, response) in futures.collect::<Vec<_>>().await {
            println!("{}: {:?}", source, response);
        }
    });
}