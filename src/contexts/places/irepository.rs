use super::domain::{Place, Filters};

#[async_trait::async_trait]
pub trait IRepository : Send + Sync
{
    async fn get_places(&self, filters: Filters) -> Vec<Place>;
}
