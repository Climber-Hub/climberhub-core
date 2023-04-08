use super::domain::{Place, Filters};

pub trait IRepository
{
    fn get_places(&self, filters: Filters) -> Vec<Place>;
}