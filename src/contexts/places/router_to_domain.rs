use super::{router, domain};

pub fn place_id(id: router::PlaceId) -> domain::PlaceId { id }
pub fn place_data(pd: router::PlaceData) -> domain::PlaceData
{
    domain::PlaceData { 
        name        : pd.name,
        description : pd.description,
        address     : pd.address,
        postcode    : pd.postcode,
        city        : pd.city,
        country     : pd.country,
     }
}

pub mod get
{
    use super::{
        router::get as router, 
        domain::get as domain,
    };

    pub fn filters(f: router::Filters) -> domain::Filters
    {
        domain::Filters {
            country : f.country,
            city    : f.city,
        }
    }
}