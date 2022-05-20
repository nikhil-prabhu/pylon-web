//! API routes definitions and configuration.

use rocket::serde::json::Json;

use crate::controllers;
use crate::core::Payload;
use crate::Response;

/// Generic index route that indicates whether the service is up and running.
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

/// Generates and returns the wormhole authentication code.
#[get("/code")]
pub async fn code() -> Json<Response<String>> {
    let code = controllers::gen_code().await;

    match code {
        Ok(code) => Json::from(Response {
            code: 200,
            message: None,
            data: Some(code),
        }),
        Err(e) => Json::from(Response {
            code: 500,
            message: Some(e.to_string()),
            data: None,
        }),
    }
}

/// Sends a payload through the encrypted wormhole tunnel.
///
/// # Arguments
///
/// * `payload` - The json payload containing the wormhole code and message to send.
#[post("/send", data = "<payload>", format = "json")]
pub async fn send(payload: Json<Payload>) -> Json<Response<()>> {
    let payload = Json::into_inner(payload);
    let res = controllers::send_payload(payload).await;

    match res {
        Ok(_) => Json::from(Response {
            code: 200,
            message: None,
            data: Some(()),
        }),
        Err(e) => Json::from(Response {
            code: 500,
            message: Some(e.to_string()),
            data: None,
        }),
    }
}

/// Receives a payload through the encrypted wormhole tunnel.
///
/// # Arguments
///
/// * `payload` - The json payload containing the wormhole code.
#[post("/receive", data = "<payload>", format = "json")]
pub async fn receive(payload: Json<Payload>) -> Json<Response<Payload>> {
    let payload = Json::into_inner(payload);
    let res = controllers::receive_payload(payload.code).await;

    match res {
        Ok(payload) => Json::from(Response {
            code: 200,
            message: None,
            data: Some(payload),
        }),
        Err(e) => Json::from(Response {
            code: 500,
            message: Some(e.to_string()),
            data: None,
        }),
    }
}
