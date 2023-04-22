use crate::contexts::routes::
{
    irepository::IRepository,
    domain::{Route, Rules, Filters},
};
pub struct Repository;
impl IRepository for Repository
{
    fn get_routes(&self, filters: Filters) -> Vec<Route> 
    {
        vec![Route {
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
        }]

    }
}