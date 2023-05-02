use super::{router, domain};

pub fn route_id(id: router::RouteId) -> domain::RouteId { id }
pub fn route_data(rd: router::RouteData) -> domain::RouteData
{
    domain::RouteData { 
        name        : rd.name,
        description : rd.description,
        grade       : rd.grade,
        color       : rd.color,
        sector      : rd.sector,
        rules       : rules(rd.rules),
        tags        : rd.tags,
        properties  : rd.properties,
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
        domain::Filters 
        {
            min_grade  : f.min_grade,
            max_grade  : f.max_grade,
            tags       : f.tags,
            properties : f.properties,
        }
    }
}

fn rules(r: router::Rules) -> domain::Rules
{
    domain::Rules 
    {
        sitstart        : r.sitstart,
        modules_allowed : r.modules_allowed,
        edges_allowed   : r.edges_allowed,
    }
}