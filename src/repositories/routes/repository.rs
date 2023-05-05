use std::collections::HashMap;
use async_trait::async_trait;

use crate::contexts::routes::
{
    irepository,
    domain::{self, Route, RouteData, RouteId, Rules}, 
};
use crate::errors::{GetError, GetAllError};

pub struct Repository;
#[async_trait]
impl irepository::get::IRepository for Repository
{
    async fn get_all(&self, filters: domain::get::Filters) -> Result<Vec<Route>, GetAllError> 
    {
        Ok(vec![Route {
            id   : String::from("0"),
            data : RouteData {
                name        : String::new(),
                description : String::new(),
                grade       : String::from("4c"),
                color       : String::from("black"),
                sector      : String::new(),
                rules       : Rules {
                    sitstart        : false,
                    modules_allowed : false,
                    edges_allowed   : false,
                },
                tags        : filters.tags.clone(),
                properties  : filters.properties.clone(),
            }
        }])
    }
    
    async fn get(&self, id: RouteId) -> Result<Route, GetError> 
    {
        // Err(NonExistingId(id))
        Ok(Route {
            id   : id,
            data : RouteData {
                name        : String::new(),
                description : String::new(),
                grade       : String::from("4c"),
                color       : String::from("black"),
                sector      : String::new(),
                rules       : Rules {
                    sitstart        : false,
                    modules_allowed : false,
                    edges_allowed   : false,
                },
                tags        : vec![],
                properties  : HashMap::new(),
            }
        })
    }
}

use crate::errors::CreateError;
#[async_trait]
impl irepository::post::IRepository for Repository
{
    async fn create(&self, route_data: RouteData) -> Result<Route, CreateError>
    {
        Ok(Route
        {
            id   : RouteId::from("0"),
            data : route_data,
        })
    }
}

use crate::errors::UpdateError;
#[async_trait]
impl irepository::put::IRepository for Repository
{
    async fn update(&self, _id: RouteId, _data: RouteData) -> Result<(), UpdateError> 
    {
        Ok(())
        // Err(NonExistingId(_id))
    }
}

use crate::errors::DeleteError;
#[async_trait]
impl irepository::delete::IRepository for Repository
{
    async fn delete(&self, _id: RouteId) -> Result<(), DeleteError> 
    {
        Ok(())
        // Err(NonExistingId(_id))
    }
}