use std::collections::HashMap;

use rocket::{get, serde::json::Json, State, FromForm};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use serde::{Deserialize, Serialize};

use super::{use_cases::UseCase, domain_to_router, router_to_domain};

/// # Get routes that match the given filters
///
/// Returns all routes that match the given filters.
#[openapi(tag = "routes")]
#[get("/routes?<filters..>")]
pub fn get_routes(filters: Filters, use_case: &State<UseCase>) -> Json<Vec<Route>> 
{
    Json(use_case.get_routes(router_to_domain::filters(filters))
        .into_iter().map(domain_to_router::route).collect())
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Route
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

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Rules
{
    pub sitstart        : bool,
    pub modules_allowed : bool,
    pub edges_allowed   : bool,
}

#[derive(FromForm, JsonSchema, Debug)]
pub struct Filters
{
    pub min_grade  : Option<String>,
    pub max_grade  : Option<String>,
    pub tags       : Vec<String>,
    pub properties : HashMap<String, String>,
}
