use super::{domain::User, irepository::IRepository};

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
    
    pub fn get_users(&self) -> Vec<User>
    {
        self.repository.get_users()
    }    
}
