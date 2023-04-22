use super::{router, domain};

pub fn filters(f: router::Filters) -> domain::Filters
{
    domain::Filters 
    {
        min_grade  : f.min_grade,
        max_grade  : f.max_grade,
        tags       : f.tags,
        properties : f.properties,
    }
}