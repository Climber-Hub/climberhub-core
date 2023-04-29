use std::collections::HashMap;

use rocket::FromForm;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

pub mod get
{
    use rocket::{get, serde::json::Json, State, FromForm, response::status::NotFound};
    use rocket_okapi::{
        openapi,
        okapi::schemars::{self, JsonSchema},
    };

    use super::super::{use_cases::get::UseCase, domain_to_router, router_to_domain};
    use super::{Route, RouteId};

    use std::collections::HashMap;

    /// # Get the route that has the given id
    ///
    /// Returns the route that has the given id.
    #[openapi(tag = "routes")]
    #[get("/routes/<id>")]
    pub fn get_route_by_id(id: RouteId, use_case: &State<UseCase>) -> Result<Json<Route>, NotFound<String>>
    {
        match use_case.get_route_by_id(router_to_domain::route_id(id))
        {
            Ok(route) => Ok(Json(domain_to_router::route(route))),
            Err(e) => Err(NotFound(format!("Route with id `{}` was not found.", e.id))),
        }
    }

    /// # Get routes that match the given filters
    ///
    /// Returns all routes that match the given filters.
    #[openapi(tag = "routes")]
    #[get("/routes?<filters..>")]
    pub fn get_routes(filters: Filters, use_case: &State<UseCase>) -> Json<Vec<Route>> 
    {
        Json(use_case.get_routes(router_to_domain::get::filters(filters))
            .into_iter().map(domain_to_router::route).collect())
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
    use rocket::{post, serde::json::Json, State};
    use rocket_okapi::openapi;

    use super::super::{use_cases::post::UseCase, domain_to_router, router_to_domain};

    use super::{RouteData, Route};

    /// # Create a new route with the given data
    ///
    /// Returns the newly created route with an associated id
    #[openapi(tag = "routes")]
    #[post("/routes", data = "<route_data>")]
    pub fn create_route(route_data: Json<RouteData>, use_case: &State<UseCase>) -> Json<Route>
    {
        Json(domain_to_router::route(
            use_case.create_route(router_to_domain::route_data(route_data.into_inner()))
        ))
    }
}

pub type RouteId = String;

#[derive(FromForm, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Route
{
    pub id          : RouteId,
    pub name        : String,
    pub description : String,
    pub grade       : String,
    pub color       : String,
    pub sector      : String,
    pub rules       : Rules,
    pub tags        : Vec<String>,
    pub properties  : HashMap<String, String>,
}

#[derive(FromForm, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RouteData
{
    pub name        : String,
    pub description : String,
    pub grade       : String,
    pub color       : String,
    pub sector      : String,
    pub rules       : Rules,
    pub tags        : Vec<String>,
    pub properties  : HashMap<String, String>,
}

#[derive(FromForm, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Rules
{
    pub sitstart        : bool,
    pub modules_allowed : bool,
    pub edges_allowed   : bool,
}
