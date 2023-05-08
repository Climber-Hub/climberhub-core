use std::collections::HashMap;
use async_trait::async_trait;

use crate::repositories::{
    common::{self, impl_identifiable_for, Identifiable, Manager, FilterList, RelativeId, FetchError},
    config::Config,
};
use crate::typeutil::repositories::Date;
use crate::contexts::routes::{irepository, domain};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Rules {
    sitstart        : bool,
    modules_allowed : bool,
    edges_allowed   : bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Route {
    pub id           : String,
    pub place_id     : String,
    pub name         : String,
    pub description  : String,
    pub grade        : String,
    pub color        : String,
    pub sector       : String,
    pub opening_date : Date,
    pub closing_date : Option<Date>,
    pub rules        : Rules,
    pub tags         : Vec<String>,
    pub properties   : HashMap<String, String>,
}
impl_identifiable_for!(Route);

mod domain_to_repository {
    use super::{domain, Date, Route, Rules, FilterList};

    pub fn route(r: domain::Route) -> Route {
        Route {
            id           : r.id,
            place_id     : r.data.place_id,
            name         : r.data.name,
            description  : r.data.description,
            grade        : r.data.grade,
            color        : r.data.color,
            sector       : r.data.sector,
            rules        : rules(r.data.rules),
            opening_date : date(r.data.opening_date),
            closing_date : r.data.closing_date.map(date),
            tags         : r.data.tags,
            properties   : r.data.properties,
        }
    }

    fn date(d: domain::Date) -> Date { Date(d) }
    fn rules(r: domain::Rules) -> Rules
    {
        Rules {
            sitstart        : r.sitstart,
            modules_allowed : r.modules_allowed,
            edges_allowed   : r.edges_allowed,
        }
    }

    pub fn get_filters(f: domain::get::Filters) -> FilterList {
        let mut filters = FilterList::new();

        if let Some(min_grade) = f.min_grade {
            filters.push((String::from("min_grade"), min_grade));
        }

        if let Some(max_grade) = f.max_grade {
            filters.push((String::from("max_grade"), max_grade));
        }

        for tag in f.tags {
            filters.push((String::from("tags"), tag));
        }

        // may need to be handled differently
        for (key, value) in f.properties {
            filters.push((format!("properties.{key}"), value));
        }

        filters
    }
}

mod repository_to_domain {
    use super::{domain, Route, Rules, HashMap, FilterList, Date};

    pub fn route(r: Route) -> domain::Route {
        domain::Route {
            id   : r.id,
            data : domain::RouteData {
                place_id    : r.place_id,
                name        : r.name,
                description : r.description,
                grade       : r.grade,
                color       : r.color,
                sector      : r.sector,
                rules       : rules(r.rules),
                opening_date: date(r.opening_date),
                closing_date: r.closing_date.map(date),
                tags        : r.tags,
                properties  : r.properties,
            }
        }
    }
    fn date(d: Date) -> domain::Date { *d }
    fn rules(r: Rules) -> domain::Rules
    {
        domain::Rules {
            sitstart        : r.sitstart,
            modules_allowed : r.modules_allowed,
            edges_allowed   : r.edges_allowed,
        }
    }

    pub fn get_filters(f: FilterList) -> domain::get::Filters {
        let mut filters = domain::get::Filters {
            min_grade  : None,
            max_grade  : None,
            tags       : Vec::new(),
            properties : HashMap::new(),
        };

        for (key, value) in f {
            match key.as_str() {
                "min_grade" => filters.min_grade = Some(value),
                "max_grade" => filters.max_grade = Some(value),
                "tags"      => filters.tags.push(value),
                // may need to be handled differently
                prop if prop.starts_with("properties.") => 
                {
                    filters.properties.insert(key["properties.".len()..].to_string(), value);
                },
                _ => panic!("Unknown filter."),
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

use crate::errors::{GetError, GetAllError};
#[async_trait]
impl irepository::get::IRepository for Repository
{
    async fn get_all(&self, filters: domain::get::Filters) -> Result<Vec<domain::Route>, GetAllError> 
    {
        let (routes, _errors) = self.manager.dispatch(
            common::path_with_filters("routes", domain_to_repository::get_filters(filters)).as_str()).await;

        if routes.is_empty() {
            Err(GetAllError::InternalServerError)
        } else {
            Ok(routes.into_iter().map(|r| repository_to_domain::route(r)).collect())
        }
    }
    
    async fn get(&self, id: domain::RouteId) -> Result<domain::Route, GetError> 
    {
        let RelativeId{source_id, resource_id} = RelativeId::from_str(id.as_str());
        match self.manager.get(source_id, format!("routes/{resource_id}").as_str()).await
        {
            Ok(route) => Ok(repository_to_domain::route(route)),
            Err(e) => 
            {
                eprintln!("{e:?}");
                match e 
                {
                    FetchError::Networking(_)    => Err(GetError::NonExistingId(id)),
                    FetchError::Serialization(_) => Err(GetError::InternalServerError),
                }
            },
        }
    }
}

use crate::errors::CreateError;
#[async_trait]
impl irepository::post::IRepository for Repository
{
    async fn create(&self, data: domain::RouteData) -> Result<domain::Route, CreateError>
    {
        unimplemented!("create")
        // Ok(domain::Route
        // {
        //     id   : domain::RouteId::from("0"),
        //     data : route_data,
        // })
    }
}

use crate::errors::UpdateError;
#[async_trait]
impl irepository::put::IRepository for Repository
{
    async fn update(&self, _id: domain::RouteId, _data: domain::RouteData) -> Result<(), UpdateError> 
    {
        unimplemented!("update")
        // Err(NonExistingId(_id))
    }
}

use crate::errors::DeleteError;
#[async_trait]
impl irepository::delete::IRepository for Repository
{
    async fn delete(&self, _id: domain::RouteId) -> Result<(), DeleteError> 
    {
        unimplemented!("delete")
        // Err(NonExistingId(_id))
    }
}