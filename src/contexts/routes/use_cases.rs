pub mod get
{
    use crate::errors::get::{Error, IdError};

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
        
        pub fn get_routes(&self, filters: Filters) -> Result<Vec<Route>, Error>
        {
            self.repository.get_routes(filters)
        }    

        pub fn get_route_by_id(&self, id: RouteId) -> Result<Route, IdError>
        {
            self.repository.get_route_by_id(id)
        }    
        
    }
}

pub mod post
{
    use crate::errors::post::Error;

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

        pub fn create_route(&self, route_data: RouteData) -> Result<Route, Error>
        {
            self.repository.create_route(route_data)
        }     
    }
}

pub mod put
{
    use crate::errors::put::Error;

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

        pub fn update_route(&self, id: RouteId, route_data: RouteData) -> Result<(), Error>
        {
            self.repository.update_route(id, route_data)
        }     
    }
}

pub mod delete
{
    use crate::errors::delete::Error;

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

        pub fn delete_route(&self, id: RouteId) -> Result<(), Error>
        {
            self.repository.delete_route(id)
        }     
    }
}