use super::{domain, router};

pub fn route(r: domain::Route) -> router::Route
{
    router::Route
    {
        id   : route_id(r.id),
        data : route_data(r.data),
    }
}

pub fn route_data(r: domain::RouteData) -> router::RouteData
{
    router::RouteData
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

fn route_id(id: domain::RouteId) -> router::RouteId { id }
fn rules(r: domain::Rules) -> router::Rules
{
    router::Rules
    {
        sitstart        : r.sitstart,
        modules_allowed : r.modules_allowed,
        edges_allowed   : r.edges_allowed,
    }
}