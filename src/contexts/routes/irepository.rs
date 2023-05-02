pub mod get
{
    use crate::errors::get::{Error, IdError};

    use super::super::domain::{get::Filters, Route, RouteId};
    pub trait IRepository
    {
        fn get_routes(&self, filters: Filters) -> Result<Vec<Route>, Error>;
        fn get_route_by_id(&self, id: RouteId) -> Result<Route, IdError>;
    }
}

pub mod post
{
    use crate::errors::post::Error;

    use super::super::domain::{RouteData, Route};
    pub trait IRepository
    {
        fn create_route(&self, route_data: RouteData) -> Result<Route, Error>;
    }
}

pub mod put
{
    use crate::errors::put::Error;

    use super::super::domain::Route;
    pub trait IRepository
    {
        fn update_route(&self, route: Route) -> Result<Route, Error>;
    }
}

pub mod delete
{
    use crate::errors::delete::Error;

    use super::super::domain::RouteId;
    pub trait IRepository
    {
        fn delete_route(&self, id: RouteId) -> Result<(), Error>;
    }
}