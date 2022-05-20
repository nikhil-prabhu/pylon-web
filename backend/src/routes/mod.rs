//! API routes definitions and configuration.

/// Generic index route that indicates whether the service is up and running.
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}
