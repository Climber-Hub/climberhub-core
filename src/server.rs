use rocket::{Rocket, Build};
use rocket_okapi::{
    openapi_get_routes,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};


fn build() -> Rocket<Build>
{
    rocket::build()
    .mount("/", openapi_get_routes![
        crate::contexts::places::router::get_places,
    ])
    .mount(
        "/docs/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    )
}

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
