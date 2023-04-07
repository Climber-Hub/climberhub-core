use super::config::Config;
use reqwest;
use futures::stream::StreamExt;

const REPO_CONFIG: Config = Config::from_file("config.toml");

pub struct RelativeId {
    pub source_id: u16,
    pub resource_id: u32,
}

impl RelativeId {
    /// Parses a string of the form `FFFF-FFFFFFFF` into a `RelativeId`.
    pub fn from_str(id: &str) -> Self {
        let mut parts = id.split('-');
        let source_id = u16::from_str_radix(parts.next().unwrap(), 16).unwrap();
        let resource_id = u32::from_str_radix(parts.next().unwrap(), 16).unwrap();
        Self {
            source_id,
            resource_id,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:04X}-{:08X}", self.source_id, self.resource_id)
    }
}

pub struct Manager {
    config: Config,
    client: reqwest::Client,
    concurrent_requests: usize
}

impl Manager {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
            concurrent_requests: 10
        }
    }

    pub async fn get(&self, source_id: u16, path: &str) -> String {
        let source = self.config.get_source(source_id).unwrap();
        let url = format!("{}{}", source.url, path);
        let response = self.client.get(&url).send().await;
        response.unwrap().text().await.unwrap()
    }

    pub async fn dispatch(&self, path: &str) {
        let bodies = futures::stream::iter(&self.config.sources)
            .map(|source| {
                let client = &self.client;
                async move {
                    let response = client.get(&source.url).send().await?;
                    response.bytes().await
                }
            })
            .buffer_unordered(self.concurrent_requests);

        bodies
            .for_each(|b| async {
                match b {
                    Ok(b) => println!("Got {} bytes", b.len()),
                    Err(e) => eprintln!("Got an error: {}", e),
                }
            })
            .await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let id = RelativeId::from_str("0199-58B583BD");
        assert_eq!(id.source_id, 409);
        assert_eq!(id.resource_id, 1488290749);
    }

    #[test]
    fn test_to_string() {
        let id = RelativeId::from_str("0199-58B583BD");
        assert_eq!(id.to_string(), "0199-58B583BD");
    }
}
