use super::{domain, router};

pub fn route(r: domain::Route) -> router::Route
{
    router::Route
    {
        id          : route_id(r.id),
        name        : r.data.name,
        description : r.data.description,
        grade       : r.data.grade,
        color       : r.data.color,
        sector      : r.data.sector,
        rules       : rules(r.data.rules),
        tags        : r.data.tags,
        properties  : r.data.properties,
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