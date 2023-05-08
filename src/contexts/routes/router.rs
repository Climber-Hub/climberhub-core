use std::collections::HashMap;

use rocket::FromForm;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

use crate::typeutil::routers::Date;

pub mod get
{
    use rocket::{get, serde::json::Json, State, FromForm, response::status::Custom, http::Status};
    use rocket_okapi::{
        openapi,
        okapi::schemars::{self, JsonSchema},
    };

    use super::super::{use_cases::get::UseCase, domain_to_router, router_to_domain};
    use super::{Route, RouteId};

    use std::collections::HashMap;

    use crate::errors::{GetAllError, GetError};

    /// # Get the route that has the given id
    ///
    /// Returns the route that has the given id.
    #[openapi(tag = "Route")]
    #[get("/routes/<id>")]
    pub async fn get_route(id: RouteId, use_case: &State<UseCase>) -> Result<Json<Route>, Custom<String>>
    {
        match use_case.get(router_to_domain::route_id(id)).await
        {
            Ok(route) => Ok(Json(domain_to_router::route(route))),
            Err(GetError::NonExistingId(id)) => Err(Custom(Status::NotFound, format!("Route with id `{id}` was not found."))),
            Err(GetError::InternalServerError) => Err(Custom(Status::InternalServerError, String::from("Internal Server Error"))),
        }
    }

    /// # Get routes that match the given filters
    ///
    /// Returns all routes that match the given filters.
    #[openapi(tag = "Route")]
    #[get("/routes?<filters..>")]
    pub async fn get_all_routes(filters: Filters, use_case: &State<UseCase>) -> Result<Json<Vec<Route>>, Custom<String>>
    {
        match use_case.get_all(router_to_domain::get::filters(filters)).await
        {
            Ok(routes) => Ok(Json(routes.into_iter().map(domain_to_router::route).collect())),
            Err(GetAllError::InternalServerError) => Err(Custom(Status::InternalServerError, String::from("Internal Server Error"))),
        } 
    }

    #[derive(FromForm, JsonSchema, Debug)]
    pub struct Filters
    {
        pub min_grade  : Option<String>,
        pub max_grade  : Option<String>,
        pub tags       : Vec<String>,
        pub properties : HashMap<String, String>,
    }

}

pub mod post
{
    use rocket::http::Status;
    use rocket::response::status::Custom;
    use rocket::{post, serde::json::Json, State};
    use rocket_okapi::openapi;

    use crate::errors::CreateError;

    use super::super::{use_cases::post::UseCase, domain_to_router, router_to_domain};

    use super::{RouteData, Route};

    /// # Create a new route with the given data
    ///
    /// Returns the newly created route with an associated id
    #[openapi(tag = "Route")]
    #[post("/routes", data = "<route_data>")]
    pub async fn create_route(route_data: Json<RouteData>, use_case: &State<UseCase>) -> Result<Json<Route>, Custom<String>>
    {
        match use_case.create(router_to_domain::route_data(route_data.into_inner())).await
        {
            Ok(route) => Ok(Json(domain_to_router::route(route))),
            Err(CreateError::InternalServerError) => Err(Custom(Status::InternalServerError, String::from("Internal Server Error"))),
        }
    }
}

pub mod put
{
    use rocket::http::Status;
    use rocket::response::status::{self, Custom};
    use rocket::{put, serde::json::Json, State};
    use rocket_okapi::openapi;

    use crate::errors::UpdateError;

    use super::super::{use_cases::put::UseCase, router_to_domain};

    use super::{RouteData, RouteId};

    /// # Update an existing route
    #[openapi(tag = "Route")]
    #[put("/routes/<id>", data = "<route_data>")]
    pub async fn update_route(id: RouteId, route_data: Json<RouteData>, use_case: &State<UseCase>) -> Result<status::NoContent, Custom<String>>
    {
        
        match use_case.update(router_to_domain::route_id(id), router_to_domain::route_data(route_data.into_inner())).await
        {
            Ok(()) => Ok(status::NoContent),
            Err(UpdateError::NonExistingId(id)) => Err(Custom(Status::NotFound, format!("No existing route with id `{id}`."))),
            Err(UpdateError::InternalServerError) => Err(Custom(Status::InternalServerError, String::from("Internal Server Error"))),
        }
    }
}

pub mod delete
{
    use rocket::http::Status;
    use rocket::response::status::{self, Custom};
    use rocket::{delete, State};
    use rocket_okapi::openapi;

    use crate::errors::DeleteError;

    use super::super::{use_cases::delete::UseCase, router_to_domain};

    use super::RouteId;

    /// # Delete an existing route
    #[openapi(tag = "Route")]
    #[delete("/routes/<id>")]
    pub async fn delete_route(id: RouteId, use_case: &State<UseCase>) -> Result<status::NoContent, Custom<String>>
    {
        
        match use_case.delete(router_to_domain::route_id(id)).await
        {
            Ok(()) => Ok(status::NoContent),
            Err(DeleteError::NonExistingId(id)) => Err(Custom(Status::NotFound, format!("No existing route with id `{id}`."))),
            Err(DeleteError::InternalServerError) => Err(Custom(Status::InternalServerError, String::from("Internal Server Error"))),
        }
    }
}


#[derive(FromForm, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Route
{
    pub id           : RouteId,
    pub place_id     : PlaceId,
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
pub type RouteId = String;
pub type PlaceId = String;

#[derive(FromForm, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RouteData
{
    pub place_id     : PlaceId,
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

#[derive(FromForm, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rules
{
    pub sitstart        : bool,
    pub modules_allowed : bool,
    pub edges_allowed   : bool,
}
