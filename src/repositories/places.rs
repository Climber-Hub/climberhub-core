use super::common::{Manager, RelativeId};

struct Place {
    id: String,
    name: String,
    description: String,
    address: String,
    postcode: String,
    city: String,
    country: String,
}

pub struct PlacesRepository {
    manager: Manager,
}

impl PlacesRepository {
    fn get(&self, id: &str) -> Option<Place> {
        let id = RelativeId::from_str(id);
        let path = format!("places/{}", id.resource_id);
        let data = self.manager.get(id.source_id, &path);
        let place: Place = serde_json::from_str(&data).unwrap();
        Some(place)
    }
}