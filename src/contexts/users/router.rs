use rocket::{get, serde::json::Json, State, FromForm};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use serde::{Deserialize, Serialize};

use super::{use_cases::UseCase, domain_to_router};

#[derive(FromForm, JsonSchema, Debug)]
pub struct Filters
{
    pub country : Option<String>,
    pub city    : Option<String>,
}

/// # Get all the users of the system
///
/// Returns all the users of the system.
#[openapi(tag = "Users")]
#[get("/users")]
pub fn get_users(use_case: &State<UseCase>) -> Json<Vec<User>> 
{
    Json(use_case.get_users()
        .into_iter().map(domain_to_router::user).collect())
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct User 
{
    pub first_name : String,
    pub last_name  : String,
    pub username   : String,
    pub email      : String,
}