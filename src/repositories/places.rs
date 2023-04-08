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
        let mut server1 = mockito::Server::new();
        let mut server2 = mockito::Server::new();

        let config = Config::from_str(&format!(
            r#"
            [[sources]]
            name = "Test 1"
            id = 1
            url = "{}"
            [[sources]]
            name = "Test 2"
            id = 2
            url = "{}"
            "#,
            server1.url(),
            server2.url(),
        ));

        server1.mock("GET", "/places")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                [
                    {
                        "id": "0001-00000001",
                        "name": "COUM",
                        "description": "Centre Omnisport Universitaire de Moulon",
                        "address": "8 rue 128",
                        "postcode": "91190",
                        "city": "Gif-sur-Yvette",
                        "country": "France"
                    },
                    {
                        "id": "0001-00000002",
                        "name": "Bibliothèque",
                        "description": "Bibliothèque universitaire",
                        "address": "8 rue 128",
                        "postcode": "91190",
                        "city": "Gif-sur-Yvette",
                        "country": "France"
                    }
                ]
                "#,
            )
            .create();

        server2.mock("GET", "/places")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"
                [
                    {
                        "id": "0002-00000001",
                        "name": "Auditorium",
                        "description": "Auditorium universitaire",
                        "address": "1 rue des étoiles",
                        "postcode": "92100",
                        "city": "Boulogne-Billancourt",
                        "country": "France"
                    },
                    {
                        "id": "0002-00000002",
                        "name": "Restaurant",
                        "description": "Restaurant universitaire",
                        "address": "18 avenue de la République",
                        "postcode": "78000",
                        "city": "Versailles",
                        "country": "France"
                    }
                ]
                "#,
            )
            .create();

        let manager = Manager::<Place>::new(config, reqwest::Client::new());

        let repo = PlacesRepository::new(manager);
        let places = repo.get_all().await;
        assert_eq!(places.len(), 4);
        println!("{:?}", places);
    }
}