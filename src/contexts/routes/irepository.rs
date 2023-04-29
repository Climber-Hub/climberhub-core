pub mod get
{
    use super::super::{
        domain::{get::Filters, Route, RouteId},
        use_cases::get::NonExistingId,
    };
    pub trait IRepository
    {
        fn get_routes(&self, filters: Filters) -> Vec<Route>;
        fn get_route_by_id(&self, id: RouteId) -> Result<Route, NonExistingId>;
    }
}

pub mod post
{
    use super::super::domain::{RouteData, Route};
    pub trait IRepository
    {
        fn create_route(&self, route_data: RouteData) -> Route;
    }
}

pub mod put
{
    use super::super::domain::Route;
    pub trait IRepository
    {
        fn update_route(&self, route: Route) -> Route;
    }
}

pub mod delete
{
    use super::super::domain::RouteId;
    pub trait IRepository
    {
        fn delete_route(&self, id: RouteId);
    }
}