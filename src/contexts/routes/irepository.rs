pub mod get
{
    use async_trait::async_trait;
    use crate::errors::get::{Error, IdError};
    use super::super::domain::{get::Filters, Route, RouteId};

    #[async_trait]
    pub trait IRepository : Send + Sync
    {
        async fn get_routes(&self, filters: Filters) -> Result<Vec<Route>, Error>;
        async fn get_route_by_id(&self, id: RouteId) -> Result<Route, IdError>;
    }
}

pub mod post
{
    use async_trait::async_trait;
    use crate::errors::post::Error;
    use super::super::domain::{RouteData, Route};

    #[async_trait]
    pub trait IRepository : Send + Sync
    {
        async fn create_route(&self, route_data: RouteData) -> Result<Route, Error>;
    }
}

pub mod put
{
    use async_trait::async_trait;
    use crate::errors::put::Error;
    use super::super::domain::{RouteId, RouteData};

    #[async_trait]
    pub trait IRepository : Send + Sync
    {
        async fn update_route(&self, id: RouteId, data: RouteData) -> Result<(), Error>;
    }
}

pub mod delete
{
    use async_trait::async_trait;
    use crate::errors::delete::Error;
    use super::super::domain::RouteId;

    #[async_trait]
    pub trait IRepository : Send + Sync
    {
        async fn delete_route(&self, id: RouteId) -> Result<(), Error>;
    }
}