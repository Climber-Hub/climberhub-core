use super::{domain, router};

pub fn place(p: domain::Place) -> router::Place
{
    router::Place {
        id          : p.id,
        name        : p.data.name,
        description : p.data.description,
        address     : p.data.address,
        postcode    : p.data.postcode,
        city        : p.data.city,
        country     : p.data.country,
    }
}
