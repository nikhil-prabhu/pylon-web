use pylon_web::routes;

use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![routes::index])
}
