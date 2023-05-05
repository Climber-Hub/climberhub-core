use std::collections::HashMap;
use crate::repositories::{
    common::{impl_identifiable_for, Identifiable, Manager},
    config::Config,
};

use crate::contexts::routes::{ irepository, domain };
use crate::errors::get::{
    IdError as GetIdError, 
    Error   as GetError,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Rules {
    sitstart        : bool,
    modules_allowed : bool,
    edges_allowed   : bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Route {
    pub id   : String,
    name        : String,
    description : String,
    grade       : String,
    color       : String,
    sector      : String,
    rules       : Rules,
    tags        : Vec<String>,
    properties  : HashMap<String, String>,
}
impl_identifiable_for!(Route);

mod domain_to_repository {
    use super::{Route, Rules};
    use crate::contexts::routes::domain;

    pub fn route(r: domain::Route) -> Route {
        Route {
            id          : r.id,
            name        : r.data.name,
            description : r.data.description,
            grade       : r.data.grade,
            color       : r.data.color,
            sector      : r.data.sector,
            rules       : Rules {
                sitstart        : r.data.rules.sitstart,
                modules_allowed : r.data.rules.modules_allowed,
                edges_allowed   : r.data.rules.edges_allowed,
            },
            tags        : r.data.tags,
            properties  : r.data.properties,
        }
    }

    pub fn get_filters(f: domain::get::Filters) -> Vec<(String, String)> {
        let mut filters = Vec::new();

        if let Some(min_grade) = f.min_grade {
            filters.push((String::from("min_grade"), min_grade));
        }

        if let Some(max_grade) = f.max_grade {
            filters.push((String::from("max_grade"), max_grade));
        }

        for tag in f.tags {
            filters.push((String::from("tag"), tag));
        }

        // may need to be handled differently
        for (key, value) in f.properties {
            filters.push((key, value));
        }

        filters
    }
}

mod repository_to_domain {
    use super::{Route, Rules, HashMap};
    use crate::contexts::routes::domain;

    pub fn route(r: Route) -> domain::Route {
        domain::Route {
            id: r.id,
            data: domain::RouteData {
                name: r.name,
                description: r.description,
                grade: r.grade,
                color: r.color,
                sector: r.sector,
                rules: domain::Rules {
                    sitstart: r.rules.sitstart,
                    modules_allowed: r.rules.modules_allowed,
                    edges_allowed: r.rules.edges_allowed,
                },
                tags: r.tags,
                properties: r.properties,
            }
        }
    }

    pub fn get_filters(f: Vec<(String, String)>) -> domain::get::Filters {
        let mut filters = domain::get::Filters {
            min_grade: None,
            max_grade: None,
            tags: Vec::new(),
            properties: HashMap::new(),
        };

        for (key, value) in f {
            match key.as_str() {
                "min_grade" => filters.min_grade = Some(value),
                "max_grade" => filters.max_grade = Some(value),
                "tag" => filters.tags.push(value),
                // may need to be handled differently
                _ => { let _ = filters.properties.insert(key, value); },
            };
        }

        filters
    }
}

pub struct Repository {
    manager: Manager<Route>,
}

impl Default for Repository {
    fn default() -> Self {
        let config = Config::from_env();
        let manager = Manager::<Route>::new(config, reqwest::Client::new());
        Self { manager }
    }
}

use tokio; // little hack to compile while waiting for async trait methods
impl irepository::get::IRepository for Repository
{
    #[tokio::main]
    async fn get_routes(&self, filters: domain::get::Filters) -> Result<Vec<domain::Route>, GetError> 
    {
        let filters = domain_to_repository::get_filters(filters);
        let path = "routes";
        let (routes, errors) = self.manager.dispatch(&path).await;

        if routes.is_empty() {
            Err(panic!("GetError doesn't have any variants yet"))
        } else {
            Ok(routes.into_iter().map(|r| repository_to_domain::route(r)).collect())
        }
    }
    
    fn get_route_by_id(&self, id: domain::RouteId) -> Result<domain::Route, GetIdError> 
    {
        unimplemented!("get_route_by_id")
        // // Err(NonExistingId(id))
        // Ok(domain::Route {
        //     id   : id,
        //     data : domain::RouteData {
        //         name        : String::new(),
        //         description : String::new(),
        //         grade       : String::from("4c"),
        //         color       : String::from("black"),
        //         sector      : String::new(),
        //         rules       : domain::Rules {
        //             sitstart        : false,
        //             modules_allowed : false,
        //             edges_allowed   : false,
        //         },
        //         tags        : vec![],
        //         properties  : HashMap::new(),
        //     }
        // })
    }
}

use crate::errors::post::Error as PostError;
impl irepository::post::IRepository for Repository
{
    fn create_route(&self, route_data: domain::RouteData) -> Result<domain::Route, PostError>
    {
        unimplemented!("create_route")
        // Ok(domain::Route
        // {
        //     id   : domain::RouteId::from("0"),
        //     data : route_data,
        // })
    }
}

use crate::errors::put::Error as PutError;
impl irepository::put::IRepository for Repository
{
    fn update_route(&self, _id: domain::RouteId, _data: domain::RouteData) -> Result<(), PutError> 
    {
        unimplemented!("update_route")
        // Ok(())
        // // Err(PutError::NonExistingId(_id))
    }
}

use crate::errors::delete::Error as DeleteError;
impl irepository::delete::IRepository for Repository
{
    fn delete_route(&self, _id: domain::RouteId) -> Result<(), DeleteError> 
    {
        unimplemented!("delete_route")
        // Ok(())
        // // Err(PutError::NonExistingId(_id))
    }
}