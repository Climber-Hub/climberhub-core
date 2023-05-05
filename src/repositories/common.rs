use super::config::{Config, Source};
use futures::stream::StreamExt;
use reqwest;

const CONCURRENT_REQUESTS: usize = 10;

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
    fn id(&mut self) -> &mut String;
}

macro_rules! impl_identifiable_for {
    ($type:ty) => {
        impl Identifiable for $type {
            fn id(&mut self) -> &mut String {
                &mut self.id
            }
        }
    };
}
pub(crate) use impl_identifiable_for;

#[derive(Debug)]
pub enum FetchError {
    Serialization(serde_json::Error),
    Networking(reqwest::Error)
}

impl From<serde_json::Error> for FetchError {
    fn from(e: serde_json::Error) -> Self {
        FetchError::Serialization(e)
    }
}

impl From<reqwest::Error> for FetchError {
    fn from(e: reqwest::Error) -> Self {
        FetchError::Networking(e)
    }
}

pub struct Manager<T> {
    config: Config,
    client: reqwest::Client,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Identifiable + serde::de::DeserializeOwned + serde::Serialize> Manager<T> {
    pub fn new(config: Config, client: reqwest::Client) -> Self {
        Self {
            config,
            client,
            _phantom: std::marker::PhantomData,
        }
    }

    fn to_absolute(&self, object: &mut T, source_id: u16) {
        *object.id() = RelativeId {
            source_id,
            resource_id: object.id().parse::<u32>().expect("Failed to parse resource id"),
        }.to_string();
    }

    fn to_relative(&self, object: &mut T) {
        *object.id() = RelativeId::from_str(&object.id()).resource_id.to_string();
    }

    pub async fn get(&self, source_id: u16, path: &str) -> Result<T, FetchError> {
        let source = self.config.get_source(source_id).expect(&format!("Couldn't find a source with id {}", source_id));
        let url = format!("{}/{}", source.url, path);
        let response = self.client.get(&url).send().await;
        
        let body = match response {
            Ok(response) => {
                match response.text().await {
                    Ok(body) => body,
                    Err(error) => {
                        eprintln!("Error: {}", error);
                        return Err(FetchError::from(error));
                    }
                }
            },
            Err(error) => {
                eprintln!("Error: {}", error);
                return Err(FetchError::from(error))
            }
        };

        let mut object: T = match serde_json::from_str(&body) {
            Ok(object) => object,
            Err(error) => {
                eprintln!("Deserialization error: {}", error);
                return Err(FetchError::from(error));
            }
        };

        self.to_absolute(&mut object, source_id);
        Ok(object)
    }

    pub async fn post(&self, source_id: u16, path: &str, object: &mut T, ) -> Result<T, FetchError> {
        let source = self.config.get_source(source_id).expect(&format!("Couldn't find a source with id {}", source_id));
        let url = format!("{}/{}", source.url, path);
        let response = self.client.post(&url).json(object).send().await;

        let body = match response {
            Ok(response) => {
                match response.text().await {
                    Ok(body) => body,
                    Err(error) => {
                        eprintln!("Error: {}", error);
                        return Err(FetchError::from(error));
                    }
                }
            },
            Err(error) => {
                eprintln!("Error: {}", error);
                return Err(FetchError::from(error))
            }
        };

        let mut object: T = match serde_json::from_str(&body) {
            Ok(object) => object,
            Err(error) => {
                eprintln!("Deserialization error: {}", error);
                return Err(FetchError::from(error));
            }
        };

        self.to_absolute(&mut object, source.id);
        Ok(object)
    }

    pub async fn delete(&self, source_id: u16, path: &str) -> Result<bool, FetchError> {
        let source = self.config.get_source(source_id).expect(&format!("Couldn't find a source with id {}", source_id));
        let url = format!("{}/{}", source.url, path);
        let response = self.client.delete(&url).send().await;

        match response {
            Ok(response) => Ok(response.status().is_success()),
            Err(error) => {
                eprintln!("Error: {}", error);
                Err(FetchError::from(error))
            }
        }
    }

    async fn get_objects(&self, source: Source, path: &str) -> Result<Vec<T>, FetchError> {
        let client = &self.client;
        let url = format!("{}/{}", source.url, path);
        let response = client.get(&url).send().await;
        let body = match response {
            Ok(response) => response.text().await,
            Err(error) => {
                eprintln!("Error: {}", error);
                Err(error)
            }
        };
        match body {
            Ok(body) => match serde_json::from_str(&body) {
                Ok(mut objects) => {
                    for object in &mut objects {
                        self.to_absolute(object, source.id);
                    }
                    Ok(objects)
                }
                Err(error) => {
                    eprintln!("Deserialization error: {}", error);
                    Err(FetchError::from(error))
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
                Err(FetchError::from(error))
            }
        }
    }

    pub async fn dispatch(&self, path: &str) -> (Vec<T>, Vec<FetchError>) {
        let results = futures::stream::iter(self.config.sources.clone())
            // create a stream of futures
            .map(|source| async { self.get_objects(source, path) })
            // execute the futures concurrently
            .buffer_unordered(CONCURRENT_REQUESTS);

        // merges the Vec<T> from the different sources into a single Vec<T>
        let (successes, failures): (Vec<T>, Vec<FetchError>) = results
            .fold((Vec::new(), Vec::new()), |mut acc, list| async {
                match list.await {
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
