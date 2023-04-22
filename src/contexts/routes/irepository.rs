use super::domain::{Route, Filters};

pub trait IRepository
{
    fn get_routes(&self, filters: Filters) -> Vec<Route>;
}