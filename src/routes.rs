use rocket::{get, serde::json::Json, Rocket, Build};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi, openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
use serde::{Deserialize, Serialize};

mod data
{
    use std::collections::HashMap;

    struct RouteRules
    {
        sitstart : bool,
        modules  : bool,
        edges    : bool,
    }
    struct Route
    {
        uuid        : String,
        name        : String,
        description : String,
        grade       : String,
        color       : String,
        sector      : String,
        rules       : RouteRules,
        tags        : Vec<String>,
        properties  : HashMap<String, String>,
    }

}

pub struct RoutesBuilder
{
    get_all_routes: fn(),
}

impl RoutesBuilder
{
    fn build() -> Rocket<Build>
    {
        rocket::build()
        .mount("/", openapi_get_routes![
            get_all_users,
        ])
        .mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
    }
}

/// # Get all users
///
/// Returns all users in the system.
#[openapi(tag = "Users")]
#[get("/user")]
fn get_all_users() -> Json<Vec<User>> 
{
    Json(vec![User {
        user_id  : 42,
        username : "bob".to_owned(),
        email    : None,
    }])
}


#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
struct User 
{
    user_id: u64,
    username: String,
    #[schemars(example = "example_email")]
    email: Option<String>,
}

fn example_email() -> &'static str { "test@example.com" }

pub fn serve()
{
    // FIXME: The `workers` value won't reflect swaps of `Rocket` in attach
    // fairings with different config values, or values from non-Rocket configs.
    // See tokio-rs/tokio#3329 for a necessary solution in `tokio`.
    rocket::tokio::runtime::Builder::new_multi_thread()
        // NOTE: graceful shutdown depends on the "rocket-worker" prefix.
        .thread_name("rocket-worker-thread")
        .enable_all()
        .build()
        .expect("create tokio runtime")
        .block_on(async move {
            let _ = build()
                .launch()
                .await;
        });
}
