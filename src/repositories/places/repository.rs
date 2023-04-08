use crate::contexts::places::
{
    irepository::IRepository,
    domain::Place,
};
pub struct Repository;
impl IRepository for Repository
{
    fn get_places(&self) -> Vec<Place> 
    {
        vec![Place { 
            id          : String::new(),
            name        : String::new(),
            description : String::new(),
            address     : String::new(),
            postcode    : String::new(),
            city        : String::new(),
            country     : String::new(),
        }]

    }
}