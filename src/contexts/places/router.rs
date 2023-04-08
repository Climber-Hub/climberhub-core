use rocket::{get, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use serde::{Deserialize, Serialize};

use super::{use_cases::UseCase, domain_to_router};

/// # Get all places
///
/// Returns all places in the system.
#[openapi(tag = "Places")]
#[get("/places")]
pub fn get_places(use_case: &State<UseCase>) -> Json<Vec<Place>> 
{
    Json(use_case.get_places()
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
