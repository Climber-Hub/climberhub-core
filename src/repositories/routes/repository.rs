use std::collections::HashMap;

use crate::contexts::routes::
{
    irepository,
    domain::{self, Route, RouteData, RouteId, Rules}, 
    use_cases::{post::AlreadyExistingId, get::NonExistingId},
};
pub struct Repository;
impl irepository::get::IRepository for Repository
{
    fn get_routes(&self, filters: domain::get::Filters) -> Vec<Route> 
    {
        vec![Route {
            id   : String::from("0"),
            data : RouteData {
                name        : String::new(),
                description : String::new(),
                grade       : String::from("4c"),
                color       : String::from("black"),
                sector      : String::new(),
                rules       : Rules {
                    sitstart        : false,
                    modules_allowed : false,
                    edges_allowed   : false,
                },
                tags        : filters.tags.clone(),
                properties  : filters.properties.clone(),
            }
        }]
    }
    
    fn get_route_by_id(&self, _id: RouteId) -> Result<RouteData, NonExistingId> 
    {
        // Err(NonExistingId { id: id })
        Ok(RouteData {
            name        : String::new(),
            description : String::new(),
            grade       : String::from("4c"),
            color       : String::from("black"),
            sector      : String::new(),
            rules       : Rules {
                sitstart        : false,
                modules_allowed : false,
                edges_allowed   : false,
            },
            tags        : vec![],
            properties  : HashMap::new(),
        })
    }
}

impl irepository::post::IRepository for Repository
{
    fn create_route(&self, route_data: RouteData) -> Result<Route, AlreadyExistingId> 
    {
        // Err(AlreadyExistingId { id: RouteId::from("0") })
        Ok(Route
        {
            id   : RouteId::from("0"),
            data : route_data,
        })
    }
}