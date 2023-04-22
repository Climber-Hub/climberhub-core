use super::{domain::{Route, Filters}, irepository::IRepository};

pub struct UseCase
{
    repository: Box<dyn IRepository>,
}
unsafe impl Send for UseCase {}
unsafe impl Sync for UseCase {}
impl UseCase 
{
    pub fn new(repo: Box<dyn IRepository>) -> Self
    {
        Self { repository: repo }
    }
    
    pub fn get_routes(&self, filters: Filters) -> Vec<Route>
    {
        self.repository.get_routes(filters)
    }    
}
