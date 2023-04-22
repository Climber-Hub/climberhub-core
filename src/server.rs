use rocket::{Rocket, Build};
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

use crate::contexts::
{
    places::use_cases::UseCase as PlaceUseCase,
    routes::use_cases::UseCase as RouteUseCase,
    users::use_cases::UseCase  as UserUseCase,
};

pub struct Server
{
    rocket_build: Rocket<Build>,
}

impl Server
{
    pub fn new(
        place_uc : PlaceUseCase,
        route_uc : RouteUseCase,
        user_uc  : UserUseCase,
    ) -> Self
    {
        Server {
            rocket_build: build()
                .manage(place_uc)
                .manage(route_uc)
                .manage(user_uc)
        }
    }

    pub fn serve(self)
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
                let _ = self.rocket_build
                    .launch()
                    .await;
            });
    }
}


fn build() -> Rocket<Build>
{
    rocket::build()
    .mount("/", openapi_get_routes![
        crate::contexts::places::router::get_places,
        crate::contexts::users::router::get_users,
        crate::contexts::routes::router::get_routes,
    ])
    .mount(
        "/docs/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    )
}