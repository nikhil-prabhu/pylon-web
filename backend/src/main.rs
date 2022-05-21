use pylon_web::fairings;
use pylon_web::routes;

use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build().attach(fairings::CORSFairing).mount(
        "/",
        routes![routes::index, routes::code, routes::send, routes::receive],
    )
}
