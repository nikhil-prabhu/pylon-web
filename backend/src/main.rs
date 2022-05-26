use pylon_web::fairings;
use pylon_web::routes;

use rocket::{launch, routes};

#[cfg(not(debug_assertions))]
use rocket::fs::FileServer;

#[cfg(not(debug_assertions))]
use rocket::log::LogLevel;

#[cfg(not(debug_assertions))]
use rocket::Config;

#[cfg(not(debug_assertions))]
use std::env;

// When run in production (release) mode, we serve the frontend's static files.
#[cfg(not(debug_assertions))]
#[launch]
fn rocket() -> _ {
    let static_dir =
        env::var("PYLON_STATIC_DIR").expect("environment variable PYLON_STATIC_DIR not set");
    rocket::build()
        .configure(Config {
            log_level: LogLevel::Normal,
            ..Config::release_default()
        })
        .attach(fairings::CORSFairing)
        .mount("/", routes![routes::code, routes::send, routes::receive])
        .mount("/", FileServer::from(static_dir))
}

// When run in debug mode, we don't serve the frontend.
#[cfg(debug_assertions)]
#[launch]
fn rocket() -> _ {
    rocket::build().attach(fairings::CORSFairing).mount(
        "/",
        routes![routes::index, routes::code, routes::send, routes::receive],
    )
}
