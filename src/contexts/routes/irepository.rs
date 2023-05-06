pub mod get
{
    use async_trait::async_trait;
    use crate::errors::{GetAllError, GetError};
    use super::super::domain::{get::Filters, Route, RouteId};

    #[async_trait]
    pub trait IRepository : Send + Sync
    {
        async fn get_all(&self, filters: Filters) -> Result<Vec<Route>, GetAllError>;
        async fn get(&self, id: RouteId) -> Result<Route, GetError>;
    }
}

pub mod post
{
    use async_trait::async_trait;
    use crate::errors::CreateError;
    use super::super::domain::{RouteData, Route};

    #[async_trait]
    pub trait IRepository : Send + Sync
    {
        async fn create(&self, data: RouteData) -> Result<Route, CreateError>;
    }
}

pub mod put
{
    use async_trait::async_trait;
    use crate::errors::UpdateError;
    use super::super::domain::{RouteId, RouteData};

    #[async_trait]
    pub trait IRepository : Send + Sync
    {
        async fn update(&self, id: RouteId, data: RouteData) -> Result<(), UpdateError>;
    }
}

pub mod delete
{
    use async_trait::async_trait;
    use crate::errors::DeleteError;
    use super::super::domain::RouteId;

    #[async_trait]
    pub trait IRepository : Send + Sync
    {
        async fn delete(&self, id: RouteId) -> Result<(), DeleteError>;
    }
}