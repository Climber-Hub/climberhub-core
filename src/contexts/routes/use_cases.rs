pub mod get
{
    use crate::errors::{GetAllError, GetError};

    use super::super::{domain::{Route, RouteId, get::Filters}, irepository::get::IRepository};
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
        
        pub async fn get_all(&self, filters: Filters) -> Result<Vec<Route>, GetAllError>
        {
            self.repository.get_all(filters).await
        }    

        pub async fn get(&self, id: RouteId) -> Result<Route, GetError>
        {
            self.repository.get(id).await
        }    
        
    }
}

pub mod post
{
    use crate::errors::CreateError;

    use super::super::{domain::{Route, RouteData}, irepository::post::IRepository};
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

        pub async fn create(&self, data: RouteData) -> Result<Route, CreateError>
        {
            self.repository.create(data).await
        }     
    }
}

pub mod put
{
    use crate::errors::UpdateError;

    use super::super::{domain::{RouteData, RouteId}, irepository::put::IRepository};
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

        pub async fn update(&self, id: RouteId, data: RouteData) -> Result<(), UpdateError>
        {
            self.repository.update(id, data).await
        }     
    }
}

pub mod delete
{
    use crate::errors::DeleteError;

    use super::super::{domain::RouteId, irepository::delete::IRepository};
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

        pub async fn delete(&self, id: RouteId) -> Result<(), DeleteError>
        {
            self.repository.delete(id).await
        }     
    }
}