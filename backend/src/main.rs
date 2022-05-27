use pylon_web::fairings;
use pylon_web::routes;

use rocket::{launch, routes};

#[cfg(not(debug_assertions))]
use rocket::fs::FileServer;

#[cfg(not(debug_assertions))]
use rocket::log::LogLevel;

use rocket::Config;

use std::env;

// When run in production (release) mode, we serve the frontend's static files.
#[cfg(not(debug_assertions))]
#[launch]
fn rocket() -> _ {
    let port = match env::var("PORT") {
        Ok(port) => port.parse::<u16>().expect("could not parse port value"),
        Err(_) => 8080,
    };
    let static_dir =
        env::var("PYLON_STATIC_DIR").expect("environment variable PYLON_STATIC_DIR not set");
    rocket::build()
        .configure(Config {
            log_level: LogLevel::Normal,
            port,
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
    let port = match env::var("PORT") {
        Ok(port) => port.parse::<u16>().expect("could not parse port value"),
        Err(_) => 8080,
    };
    rocket::build()
        .configure(Config {
            port,
            ..Config::debug_default()
        })
        .attach(fairings::CORSFairing)
        .mount(
            "/",
            routes![routes::index, routes::code, routes::send, routes::receive],
        )
}
