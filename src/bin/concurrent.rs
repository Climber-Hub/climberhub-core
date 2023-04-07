use std::fs;
use serde::Deserialize;
use reqwest::Client as WebClient;
use futures::StreamExt;


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

const CONCURRENT_REQUESTS: usize = 2;

// https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest
#[tokio::main]
async fn main() {
    let contents = fs::read_to_string("config.toml")
        .expect("Something went wrong reading the file");

    // read config as Config struct from contents
    let config: Config = toml::from_str(&contents).unwrap();

    // print config
    println!("{:#?}", config);

    let web_client = WebClient::new();

    let bodies = futures::stream::iter(config.sources)
    .map(|source| {
        let client = &web_client;
        async move {
            let response = client.get(source.url).send().await?;
            response.bytes().await
        }
    })
    .buffer_unordered(CONCURRENT_REQUESTS);

    bodies.for_each(|b| async {
        match b {
            Ok(b) => println!("Got {} bytes", b.len()),
            Err(e) => eprintln!("Got an error: {}", e),
        }
    })
    .await;
}