use rocket::{Rocket, Build};
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

use crate::contexts::
{
    places::use_cases::UseCase       as PlaceUseCase,
    routes::use_cases::get::UseCase  as RouteGetUseCase,
    routes::use_cases::post::UseCase as RoutePostUseCase,
    routes::use_cases::put::UseCase  as RoutePutUseCase,
    users::use_cases::UseCase        as UserUseCase,
};

pub struct Server
{
    rocket_build: Rocket<Build>,
}

impl Server
{
    pub fn new(
        place_uc      : PlaceUseCase,
        route_get_uc  : RouteGetUseCase,
        route_post_uc : RoutePostUseCase,
        route_put_uc  : RoutePutUseCase,
        user_uc       : UserUseCase,
    ) -> Self
    {
        Server {
            rocket_build: build()
                .manage(place_uc)
                .manage(route_get_uc)
                .manage(route_post_uc)
                .manage(route_put_uc)
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
        crate::contexts::routes::router::get::get_routes,
        crate::contexts::routes::router::get::get_route_by_id,
        crate::contexts::routes::router::post::create_route,
        crate::contexts::routes::router::put::update_route,
    ])
    .mount(
        "/docs/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    )
}