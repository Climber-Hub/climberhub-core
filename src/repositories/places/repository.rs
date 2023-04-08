use crate::contexts::places::
{
    irepository::IRepository,
    domain::{Place, Filters},
};
pub struct Repository;
impl IRepository for Repository
{
    fn get_places(&self, filters: Filters) -> Vec<Place> 
    {
        vec![Place { 
            id          : String::new(),
            name        : String::new(),
            description : String::new(),
            address     : String::new(),
            postcode    : String::new(),
            city        : filters.city.unwrap_or(String::new()),
            country     : filters.country.unwrap_or(String::new()),
        }]

    }
}