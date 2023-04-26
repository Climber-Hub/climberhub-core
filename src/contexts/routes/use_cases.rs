pub mod get
{
    use super::super::{domain::{Route, RouteId, RouteData, get::Filters}, irepository::get::IRepository};
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

        pub fn get_route_by_id(&self, id: RouteId) -> Result<RouteData, NonExistingId>
        {
            self.repository.get_route_by_id(id)
        }    
        
    }
    pub struct NonExistingId
    {
        pub id: RouteId,
    }
}

pub mod post
{
    use super::super::{domain::{Route, RouteData, RouteId}, irepository::post::IRepository};
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

        pub fn create_route(&self, route_data: RouteData) -> Result<Route, AlreadyExistingId>
        {
            self.repository.create_route(route_data)
        }     
    }

    pub struct AlreadyExistingId
    {
        pub id: RouteId,
    }
}