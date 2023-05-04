use super::{domain::{Place, Filters}, irepository::IRepository};

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
    
    pub async fn get_places(&self, filters: Filters) -> Vec<Place>
    {
        self.repository.get_places(filters).await
    }    
}
