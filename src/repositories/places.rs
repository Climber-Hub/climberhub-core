use super::common::{Manager, RelativeId, CONFIG_PATH};
use super::config::Config;

#[derive(Debug, serde::Deserialize)]
pub struct Place {
    pub id: String,
    pub name: String,
    pub description: String,
    pub address: String,
    pub postcode: String,
    pub city: String,
    pub country: String,
}

pub struct PlacesRepository {
    manager: Manager<Place>,
}

impl PlacesRepository {
    pub fn new() -> Self {
        Self {
            manager: Manager::new(Config::from_file(CONFIG_PATH), reqwest::Client::new()),
        }
    }

    pub async fn get(&self, id: &str) -> Option<Place> {
        let id = RelativeId::from_str(id);
        let path = format!("places/{}", id.resource_id);
        self.manager.get(id.source_id, &path).await
    }

    pub async fn get_all(&self) -> Vec<Place> {
        let path = "places";
        self.manager.dispatch(&path).await.0
    }
}