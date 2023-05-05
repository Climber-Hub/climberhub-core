pub mod get
{
    use async_trait::async_trait;
    use crate::errors::{GetAllError, GetError};
    use super::super::domain::{get::Filters, Place, PlaceId};

    #[async_trait]
    pub trait IRepository : Send + Sync
    {
        async fn get_all(&self, filters: Filters) -> Result<Vec<Place>, GetAllError>;
        async fn get(&self, id: PlaceId) -> Result<Place, GetError>;
    }
}

pub mod post
{
    use async_trait::async_trait;
    use crate::errors::CreateError;
    use super::super::domain::{PlaceData, Place};

    #[async_trait]
    pub trait IRepository : Send + Sync
    {
        async fn create(&self, data: PlaceData) -> Result<Place, CreateError>;
    }
}

pub mod put
{
    use async_trait::async_trait;
    use crate::errors::UpdateError;
    use super::super::domain::{PlaceId, PlaceData};

    #[async_trait]
    pub trait IRepository : Send + Sync
    {
        async fn update(&self, id: PlaceId, data: PlaceData) -> Result<(), UpdateError>;
    }
}

pub mod delete
{
    use async_trait::async_trait;
    use crate::errors::DeleteError;
    use super::super::domain::PlaceId;

    #[async_trait]
    pub trait IRepository : Send + Sync
    {
        async fn delete(&self, id: PlaceId) -> Result<(), DeleteError>;
    }
}