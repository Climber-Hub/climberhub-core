use crate::contexts::places::{domain, irepository::IRepository};
use crate::repositories::{
    common::{impl_identifiable_for, Identifiable, Manager},
    config::Config,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Place {
    pub id: String,
    pub name: String,
    pub description: String,
    pub address: String,
    pub postcode: String,
    pub city: String,
    pub country: String,
}
impl_identifiable_for!(Place);

mod repository_to_domain {
    use super::Place;
    use crate::contexts::places::domain;

    pub fn place(p: Place) -> domain::Place {
        domain::Place {
            id: p.id,
            name: p.name,
            description: p.description,
            address: p.address,
            postcode: p.postcode,
            city: p.city,
            country: p.country,
        }
    }

    pub fn filters(f: Vec<(String, String)>) -> domain::Filters {
        let mut filters = domain::Filters {
            country: None,
            city: None,
        };

        for i in f {
            match i.0.as_ref() {
                "country" => filters.country = Some(i.1),
                "city" => filters.city = Some(i.1),
                _ => (),
            }
        }

        filters
    }
}
mod domain_to_repository {
    use super::Place;
    use crate::contexts::places::domain;

    pub fn place(p: domain::Place) -> Place {
        Place {
            id: p.id,
            name: p.name,
            description: p.description,
            address: p.address,
            postcode: p.postcode,
            city: p.city,
            country: p.country,
        }
    }

    pub fn filters(f: domain::Filters) -> Vec<(String, String)> {
        let mut filters = Vec::new();

        if let Some(country) = f.country {
            filters.push(("country".to_string(), country));
        }
        if let Some(city) = f.city {
            filters.push(("city".to_string(), city));
        }

        filters
    }
}

pub struct Repository {
    manager: Manager<Place>,
}

impl Default for Repository {
    fn default() -> Self {
        let config = Config::from_env();
        let manager = Manager::<Place>::new(config, reqwest::Client::new());
        Self { manager }
    }
}

#[async_trait::async_trait]
impl IRepository for Repository {
    async fn get_places(&self, filters: domain::Filters) -> Vec<domain::Place> {
        let path = "places";
        let (places, errors) = self.manager.dispatch(&path).await;

        // print errors to stderr
        for e in errors {
            eprintln!("{:#?}", e);
        }

        places
            .into_iter()
            .map(|p|  repository_to_domain::place(p.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::{common::RelativeId, config::Source};

    impl Repository {
        pub async fn get_all(&self) -> Vec<Place> {
            return self
                .get_places(domain::Filters {
                    country: None,
                    city: None,
                }).await
                .into_iter()
                .map(|p| domain_to_repository::place(p))
                .collect()
        }
    }

    // #[tokio::test]
    // async fn test_get() {
    //     let mut server = mockito::Server::new();

    //     let config = Config::from_str(&format!(
    //         r#"
    //         [[sources]]
    //         name = "Test"
    //         id = 1
    //         url = "{}"
    //         "#,
    //         server.url(),
    //     ));

    //     server
    //         .mock("GET", "/places/1")
    //         .with_status(200)
    //         .with_header("content-type", "application/json")
    //         .with_body(
    //             r#"
    //             {
    //                 "id": "00000001",
    //                 "name": "COUM",
    //                 "description": "Centre Omnisport Universitaire de Moulon",
    //                 "address": "8 rue 128",
    //                 "postcode": "91190",
    //                 "city": "Gif-sur-Yvette",
    //                 "country": "France"
    //             }
    //             "#,
    //         )
    //         .create();

    //     let manager = Manager::<Place>::new(config, reqwest::Client::new());

    //     let repo = Repository { manager };
    //     let place = repo.get("0001-00000001").await.unwrap();
    //     assert_eq!(place.id, "0001-00000001");
    //     assert_eq!(place.name, "COUM");
    //     assert_eq!(
    //         place.description,
    //         "Centre Omnisport Universitaire de Moulon"
    //     );
    //     assert_eq!(place.address, "8 rue 128");
    //     assert_eq!(place.postcode, "91190");
    //     assert_eq!(place.city, "Gif-sur-Yvette");
    //     assert_eq!(place.country, "France");
    // }

    #[tokio::test]
    async fn test_get_all() {
        const SOURCES_COUNT: usize = 2;
        let mut servers: [mockito::ServerGuard; SOURCES_COUNT] =
            core::array::from_fn(|_| mockito::Server::new());

        let config = Config {
            sources: servers
                .iter()
                .enumerate()
                .map(|(i, server)| Source {
                    name: format!("Source {}", i + 1),
                    id: (i + 1) as u16,
                    url: server.url(),
                })
                .collect(),
        };

        const PLACES_COUNT: usize = 2 * SOURCES_COUNT;
        let expected_places: [Place; PLACES_COUNT] = core::array::from_fn(|i| {
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
            // places_slice is a slice of the expected_places array with places id mapped to relative id
            let places_slice = &expected_places[i * 2..(i + 1) * 2]
                .iter()
                .map(|place| Place {
                    id: RelativeId::from_str(&place.id).resource_id.to_string(),
                    ..place.clone()
                })
                .collect::<Vec<Place>>();
            println!("{}: {:?}", i + 1, places_slice);

            server
                .mock("GET", "/places")
                .with_status(200)
                .with_header("content-type", "application/json")
                .with_body(serde_json::to_string(places_slice).unwrap())
                .create();
        }

        let manager = Manager::<Place>::new(config, reqwest::Client::new());
        let repo = Repository { manager };

        let places = repo.get_all().await;

        // assert all places are present
        assert_eq!(places.len(), PLACES_COUNT);
        for expected_place in &expected_places {
            assert!(places.contains(&expected_place));
        }

        println!("{:?}", places);
    }
}
