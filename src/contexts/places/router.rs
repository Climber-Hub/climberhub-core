use rocket::{get, serde::json::Json, State, FromForm};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use serde::{Deserialize, Serialize};

use super::{use_cases::UseCase, domain_to_router, router_to_domain};

#[derive(FromForm, JsonSchema, Debug)]
pub struct Filters
{
    pub country : Option<String>,
    pub city    : Option<String>,
}

/// # Get places that match the given filters
///
/// Returns all places that match the given filters.
#[openapi(tag = "Places")]
#[get("/places?<filters..>")]
pub fn get_places(filters: Filters, use_case: &State<UseCase>) -> Json<Vec<Place>> 
{
    Json(use_case.get_places(router_to_domain::filters(filters))
        .into_iter().map(domain_to_router::place).collect())
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Place 
{
    pub id          : String,
    pub name        : String,
    pub description : String,
    pub address     : String,
    pub postcode    : String,
    pub city        : String,
    pub country     : String,
}
