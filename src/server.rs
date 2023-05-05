use rocket::{Rocket, Build};
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};

use crate::contexts::
{
    places::use_cases::get::UseCase    as PlaceGetUseCase,
    routes::use_cases::get::UseCase    as RouteGetUseCase,
    routes::use_cases::post::UseCase   as RoutePostUseCase,
    routes::use_cases::put::UseCase    as RoutePutUseCase,
    routes::use_cases::delete::UseCase as RouteDeleteUseCase,
};

pub struct Server
{
    rocket_build: Rocket<Build>,
}

impl Server
{
    pub fn new(
        place_get_uc  : PlaceGetUseCase,
        route_get_uc  : RouteGetUseCase,
        route_post_uc : RoutePostUseCase,
        route_put_uc  : RoutePutUseCase,
        route_del_uc  : RouteDeleteUseCase,
    ) -> Self
    {
        Server {
            rocket_build: build()
                .manage(place_get_uc)
                .manage(route_get_uc)
                .manage(route_post_uc)
                .manage(route_put_uc)
                .manage(route_del_uc)
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
        crate::contexts::places::router::get::get_all_places,
        crate::contexts::routes::router::get::get_all_routes,
        crate::contexts::routes::router::get::get_route,
        crate::contexts::routes::router::post::create_route,
        crate::contexts::routes::router::put::update_route,
        crate::contexts::routes::router::delete::delete_route,
    ])
    .mount(
        "/docs/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    )
}