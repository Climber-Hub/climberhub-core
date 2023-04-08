use rocket::{get, serde::json::Json};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use serde::{Deserialize, Serialize};

/// # Get all places
///
/// Returns all places in the system.
#[openapi(tag = "Places")]
#[get("/places")]
pub fn get_places() -> Json<Vec<Place>> 
{
    Json(vec![Place { 
        id          : String::new(),
        name        : String::new(),
        description : String::new(),
        address     : String::new(),
        postcode    : String::new(),
        city        : String::new(),
        country     : String::new(),
    }])
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Place 
{
    id          : String,
    name        : String,
    description : String,
    address     : String,
    postcode    : String,
    city        : String,
    country     : String,
}
