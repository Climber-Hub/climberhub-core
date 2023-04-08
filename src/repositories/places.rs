use super::common::{Manager, RelativeId, CONFIG_PATH};
use super::config::{Config, Source};


#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq)]
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
    pub fn new(manager: Manager<Place>) -> Self {
        Self {
            manager: manager,
        }
    }

    pub fn default() -> Self {
        let config = Config::from_file(CONFIG_PATH);
        let manager = Manager::<Place>::new(config, reqwest::Client::new());
        Self::new(manager)
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

// write some unit tests with mockito
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() {
        let mut server = mockito::Server::new();

        let config = Config::from_str(&format!(
            r#"
            [[sources]]
            name = "Test"
            id = 1
            url = "{}"
            "#,
            server.url(),
        ));
        
        server.mock("GET", "/places/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                {
                    "id": "0001-00000001",
                    "name": "COUM",
                    "description": "Centre Omnisport Universitaire de Moulon",
                    "address": "8 rue 128",
                    "postcode": "91190",
                    "city": "Gif-sur-Yvette",
                    "country": "France"
                }
                "#,
            )
            .create();

        let manager = Manager::<Place>::new(config, reqwest::Client::new());

        let repo = PlacesRepository::new(manager);
        let place = repo.get("0001-00000001").await.unwrap();
        assert_eq!(place.id, "0001-00000001");
        assert_eq!(place.name, "COUM");
        assert_eq!(place.description, "Centre Omnisport Universitaire de Moulon");
        assert_eq!(place.address, "8 rue 128");
        assert_eq!(place.postcode, "91190");
        assert_eq!(place.city, "Gif-sur-Yvette");
        assert_eq!(place.country, "France");
    }

    #[tokio::test]
    async fn test_get_all() {
        const SOURCES_COUNT: usize = 2;
        let mut servers: [mockito::ServerGuard; SOURCES_COUNT] = core::array::from_fn(|_| mockito::Server::new());

        let config = Config {
            sources: servers.iter().enumerate().map(|(i, server)| {
                Source {
                    name: format!("Source {}", i),
                    id: i as u16,
                    url: server.url(),
                }
            }).collect(),
        };

        const PLACES_COUNT: usize = 2 * SOURCES_COUNT;
        let places: [Place; PLACES_COUNT] = core::array::from_fn(|i| {
            let source_id = i / 2;
            let resource_id = i % 2;
            Place {
                id: format!("000{}-0000000{}", source_id + 1, resource_id + 1),
                name: format!("Place {}", i),
                description: format!("Description {}", i),
                address: format!("Address {}", i),
                postcode: format!("Postcode {}", i),
                city: format!("City {}", i),
                country: format!("Country {}", i),
            }
        });

        // each server will get a slice of the places array
        for (i, server) in servers.iter_mut().enumerate() {
            let places_slice = &places[i * 2..(i + 1) * 2];
            server.mock("GET", "/places")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(serde_json::to_string(places_slice).unwrap())
                .create();
        }

        let manager = Manager::<Place>::new(config, reqwest::Client::new());
        let repo = PlacesRepository::new(manager);

        let places = repo.get_all().await;
        
        // assert all places are present
        assert_eq!(places.len(), PLACES_COUNT);
        for place in &places {
            assert!(places.contains(&place));
        }

        println!("{:?}", places);
    }
}