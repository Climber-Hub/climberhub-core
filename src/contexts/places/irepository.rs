use super::domain::Place;

pub trait IRepository
{
    fn get_places(&self) -> Vec<Place>;
}