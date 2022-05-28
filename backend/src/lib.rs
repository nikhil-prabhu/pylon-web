//! A basic REST API service.

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

use std::error::Error;

use serde::{Deserialize, Serialize};

pub mod consts;
pub mod controllers;
pub mod core;
pub mod fairings;
pub mod routes;

/// A structured API response.
#[derive(Serialize, Deserialize)]
pub struct Response<S: Serialize> {
    /// The HTTP status code.
    pub code: u16,

    /// An optional message.
    pub message: Option<String>,

    /// An optional data payload.
    pub data: Option<S>,
}

/// A thread-safe error.
pub type ThreadSafeError = Box<dyn Error + Send + Sync>;
