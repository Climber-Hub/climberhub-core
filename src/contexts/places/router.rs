use rocket::FromForm;
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::{Deserialize, Serialize};

pub mod get
{
    use rocket::{get, serde::json::Json, State, FromForm, response::status::Custom, http::Status};
    use rocket_okapi::{
        openapi,
        okapi::schemars::{self, JsonSchema},
    };

    use super::super::{use_cases::get::UseCase, domain_to_router, router_to_domain};
    use super::{Place, PlaceId};

    use crate::errors::{GetAllError, GetError};

    /// # Get the place that has the given id
    ///
    /// Returns the place that has the given id.
    #[openapi(tag = "Places")]
    #[get("/places/<id>")]
    pub async fn get_place(id: PlaceId, use_case: &State<UseCase>) -> Result<Json<Place>, Custom<String>>
    {
        match use_case.get(router_to_domain::place_id(id)).await
        {
            Ok(place) => Ok(Json(domain_to_router::place(place))),
            Err(GetError::NonExistingId(id)) => Err(Custom(Status::NotFound, format!("Place with id `{id}` was not found."))),
            Err(GetError::InternalServerError) => Err(Custom(Status::InternalServerError, String::from("Internal Server Error"))),
        }
    }

    /// # Get places that match the given filters
    ///
    /// Returns all places that match the given filters.
    #[openapi(tag = "Places")]
    #[get("/places?<filters..>")]
    pub async fn get_all_places(filters: Filters, use_case: &State<UseCase>) -> Result<Json<Vec<Place>>, Custom<String>>
    {
        match use_case.get_all(router_to_domain::get::filters(filters)).await
        {
            Ok(places) => Ok(Json(places.into_iter().map(domain_to_router::place).collect())),
            Err(GetAllError::InternalServerError) => Err(Custom(Status::InternalServerError, String::from("Internal Server Error"))),
        } 
    }

    #[derive(FromForm, JsonSchema, Debug)]
    pub struct Filters
    {
        pub country : Option<String>,
        pub city    : Option<String>,
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

    use super::{PlaceData, Place};

    /// # Create a new place with the given data
    ///
    /// Returns the newly created place with an associated id
    #[openapi(tag = "Places")]
    #[post("/places", data = "<route_data>")]
    pub async fn create_place(route_data: Json<PlaceData>, use_case: &State<UseCase>) -> Result<Json<Place>, Custom<String>>
    {
        match use_case.create(router_to_domain::place_data(route_data.into_inner())).await
        {
            Ok(place) => Ok(Json(domain_to_router::place(place))),
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

    use super::{PlaceData, PlaceId};

    /// # Update an existing place
    #[openapi(tag = "Places")]
    #[put("/places/<id>", data = "<route_data>")]
    pub async fn update_place(id: PlaceId, route_data: Json<PlaceData>, use_case: &State<UseCase>) -> Result<status::NoContent, Custom<String>>
    {
        
        match use_case.update(router_to_domain::place_id(id), router_to_domain::place_data(route_data.into_inner())).await
        {
            Ok(()) => Ok(status::NoContent),
            Err(UpdateError::NonExistingId(id)) => Err(Custom(Status::NotFound, format!("No existing place with id `{id}`."))),
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

    use super::PlaceId;

    /// # Delete an existing place
    #[openapi(tag = "Places")]
    #[delete("/places/<id>")]
    pub async fn delete_place(id: PlaceId, use_case: &State<UseCase>) -> Result<status::NoContent, Custom<String>>
    {
        
        match use_case.delete(router_to_domain::place_id(id)).await
        {
            Ok(()) => Ok(status::NoContent),
            Err(DeleteError::NonExistingId(id)) => Err(Custom(Status::NotFound, format!("No existing place with id `{id}`."))),
            Err(DeleteError::InternalServerError) => Err(Custom(Status::InternalServerError, String::from("Internal Server Error"))),
        }
    }
}

pub type PlaceId = String;

#[derive(FromForm, JsonSchema, Debug)]
pub struct Filters
{
    pub country : Option<String>,
    pub city    : Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Place 
{
    pub id          : PlaceId,
    pub name        : String,
    pub description : String,
    pub address     : String,
    pub postcode    : String,
    pub city        : String,
    pub country     : String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaceData
{
    pub name        : String,
    pub description : String,
    pub address     : String,
    pub postcode    : String,
    pub city        : String,
    pub country     : String,
}
