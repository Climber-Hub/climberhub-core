pub mod get
{
    use crate::errors::{GetAllError, GetError};

    use super::super::{domain::{Place, PlaceId, get::Filters}, irepository::get::IRepository};
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
        
        pub async fn get_all(&self, filters: Filters) -> Result<Vec<Place>, GetAllError>
        {
            self.repository.get_all(filters).await
        }    

        pub async fn get(&self, id: PlaceId) -> Result<Place, GetError>
        {
            self.repository.get(id).await
        }    
        
    }
}

pub mod post
{
    use crate::errors::CreateError;

    use super::super::{domain::{Place, PlaceData}, irepository::post::IRepository};
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

        pub async fn create(&self, data: PlaceData) -> Result<Place, CreateError>
        {
            self.repository.create(data).await
        }     
    }
}

pub mod put
{
    use crate::errors::UpdateError;

    use super::super::{domain::{PlaceData, PlaceId}, irepository::put::IRepository};
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

        pub async fn update(&self, id: PlaceId, data: PlaceData) -> Result<(), UpdateError>
        {
            self.repository.update(id, data).await
        }     
    }
}

pub mod delete
{
    use crate::errors::DeleteError;

    use super::super::{domain::PlaceId, irepository::delete::IRepository};
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

        pub async fn delete(&self, id: PlaceId) -> Result<(), DeleteError>
        {
            self.repository.delete(id).await
        }     
    }
}