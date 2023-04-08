use super::{router, domain};

pub fn filters(f: router::Filters) -> domain::Filters
{
    domain::Filters {
        country : f.country,
        city    : f.city,
    }
}