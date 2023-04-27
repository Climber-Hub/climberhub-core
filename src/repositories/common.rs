use super::config::Config;
use reqwest;
use futures::stream::StreamExt;

const CONCURRENT_REQUESTS: usize = 10;
pub const CONFIG_PATH: &str = "config.toml";

pub struct RelativeId {
    pub source_id: u16,
    pub resource_id: u32,
}

impl RelativeId {
    /// Parses a string of the form `FFFF-FFFFFFFF` into a `RelativeId`.
    pub fn from_str(id: &str) -> Self {
        let mut parts = id.split('-');
        let source_id = u16::from_str_radix(parts.next().unwrap(), 16).expect("Failed to parse source id");
        let resource_id = u32::from_str_radix(parts.next().unwrap(), 16).expect("Failed to parse resource id");
        Self {
            source_id,
            resource_id,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:04X}-{:08X}", self.source_id, self.resource_id)
    }
}

pub trait Identifiable {
    fn get_id(&self) -> String;
    fn set_id(&mut self, id: String);
}

pub struct Manager<T> {
    config: Config,
    client: reqwest::Client,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Identifiable + for<'de> serde::de::Deserialize<'de>> Manager<T> {
    pub fn new(config: Config, client: reqwest::Client) -> Self {
        Self {
            config,
            client,
            _phantom: std::marker::PhantomData
        }
    }

    fn to_absolute(&self, object: &mut T, source_id: u16) {
        object.set_id(RelativeId {
            source_id,
            resource_id: object.get_id().parse::<u32>().expect("Failed to parse id"),
        }.to_string());
    }

    fn to_relative(&self, object: &mut T) {
        let relative_id = RelativeId::from_str(&object.get_id());
        object.set_id(relative_id.resource_id.to_string());
    }

    pub async fn get(&self, source_id: u16, path: &str) -> Option<T> {
        let source = self.config.get_source(source_id).unwrap();
        let url = format!("{}/{}", source.url, path);
        let response = self.client.get(&url).send().await;
        let body = response.unwrap().text().await.unwrap();

        let mut object: T = serde_json::from_str(&body).unwrap();
        self.to_absolute(&mut object, source_id);
        Some(object)
    }

    pub async fn dispatch(&self, path: &str) -> (Vec<T>, Vec<reqwest::Error>) {
        let results = futures::stream::iter(&self.config.sources)
            // create a stream of futures
            .map(|source| {
                let client = &self.client;
                async move {
                    let url = format!("{}/{}", source.url, path);
                    let response = client.get(&url).send().await;
                    let body = match response {
                        Ok(response) => response.text().await,
                        Err(error) => {
                            println!("Error: {}", error);
                            Err(error)
                        },
                    };
                    match body {
                        Ok(body) => {
                            let mut objects: Vec<T> = serde_json::from_str(&body).unwrap();
                            for object in &mut objects {
                                self.to_absolute(object, source.id);
                            }
                            Ok(objects)
                        },
                        Err(error) => {
                            println!("Error: {}", error);
                            Err(error)
                        },
                    }
                }
            })
            // execute the futures concurrently
            .buffer_unordered(CONCURRENT_REQUESTS);

        // merges the Vec<T> from the different sources into a single Vec<T>
        let (successes, failures): (Vec<T>, Vec<reqwest::Error>) = results
            .fold((Vec::new(), Vec::new()), |mut acc, list| async move {
                match list {
                    Ok(list) => acc.0.extend(list),
                    Err(error) => acc.1.push(error),
                }
                acc
            })
            .await;

        (successes, failures)
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
