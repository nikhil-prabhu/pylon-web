//! A basic REST API service.

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use serde::Serialize;

#[cfg(debug_assertions)]
use serde::Deserialize;

pub mod consts;
pub mod controllers;
pub mod core;
pub mod fairings;
pub mod routes;

/// A structured API response.
#[derive(Serialize)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct Response<S: Serialize> {
    /// The HTTP status code.
    pub code: u16,

    /// An optional message.
    pub message: Option<String>,

    /// An optional data payload.
    pub data: Option<S>,
}
