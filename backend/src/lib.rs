//! A basic REST API service.

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use serde::Serialize;

pub mod consts;
pub mod controllers;
pub mod core;
pub mod fairings;
pub mod routes;

/// A structured API response.
///
/// # Fields
///
/// * `code` - The HTTP status code.
/// * `message` - An optional message.
/// * `data` - An optional data payload.
#[derive(Serialize)]
pub struct Response<S: Serialize> {
    code: u16,
    message: Option<String>,
    data: Option<S>,
}
