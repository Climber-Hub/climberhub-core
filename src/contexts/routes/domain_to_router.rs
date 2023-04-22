use super::{domain, router};

pub fn route(r: domain::Route) -> router::Route
{
    router::Route 
    {
        name        : r.name,
        description : r.description,
        grade       : r.grade,
        color       : r.color,
        sector      : r.sector,
        rules       : rules(r.rules),
        tags        : r.tags,
        properties  : r.properties,
    }
}

fn rules(r: domain::Rules) -> router::Rules
{
    router::Rules
    {
        sitstart        : r.sitstart,
        modules_allowed : r.modules_allowed,
        edges_allowed   : r.edges_allowed,
    }
}