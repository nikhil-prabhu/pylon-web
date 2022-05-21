//! API route controllers.

use std::collections::HashMap;
use std::error::Error;
use std::time::SystemTime;

use futures::lock::Mutex;

use crate::core::{Mode, Payload, Pylon, PylonError};

lazy_static! {
    static ref PYLON_MAP: Mutex<HashMap<String, Pylon>> = Mutex::new(HashMap::new());
}

async fn get_pylon() -> Result<Pylon, Box<dyn Error>> {
    Ok(Pylon::new(Mode::Sender, None).await.unwrap())
}

/// Generates a wormhole code.
/// The newly created FutureConn will be pushed into a global Pylon map to be re-used later.
pub async fn gen_code() -> Result<String, Box<dyn Error>> {
    let pylon = get_pylon().await.unwrap();
    let code = pylon.code.clone();

    if let Some(code) = code {
        let mut pylon_map = PYLON_MAP.lock().await;
        pylon_map.insert(code.clone(), pylon);

        return Ok(code);
    }

    Ok(String::new())
}

/// Sends a payload through an encrypted wormhole tunnel.
///
/// # Arguments
///
/// * `payload` - The payload to send.
pub async fn send_payload(mut payload: Payload) -> Result<(), Box<dyn Error>> {
    let mut pylon_map = PYLON_MAP.lock().await;
    let pylon = pylon_map.remove(&payload.code);

    if let Some(pylon) = pylon {
        payload.time = Some(SystemTime::now());

        if let Some(message) = &payload.message {
            payload.size = Some(message.len());
        }

        pylon.activate(Some(&payload)).await.unwrap();
    }

    Ok(())
}

/// Receives a payload through an encrypted wormhole tunnel.
///
/// # Arguments
///
/// * `code` - The wormhole code to use for PAKE authentication.
pub async fn receive_payload(code: String) -> Result<Payload, Box<dyn Error>> {
    let pylon = Pylon::new(Mode::Receiver, Some(code)).await.unwrap();
    let payload = pylon.activate(None).await.unwrap();

    if let Some(payload) = payload {
        Ok(payload)
    } else {
        Err(Box::new(PylonError("Received empty payload".into())))
    }
}
