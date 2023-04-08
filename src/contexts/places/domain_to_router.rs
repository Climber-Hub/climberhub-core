use super::{domain, router};

pub fn place(p: domain::Place) -> router::Place
{
    router::Place {
        id          : p.id,
        name        : p.name,
        description : p.description,
        address     : p.address,
        postcode    : p.postcode,
        city        : p.city,
        country     : p.country,
    }
}