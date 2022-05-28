//! API route controllers.

use std::collections::HashMap;
use std::time::SystemTime;

use futures::lock::Mutex;

use unic_segment::Graphemes;

use sha256::digest;

use crate::core::{Mode, Payload, Pylon, PylonError};
use crate::ThreadSafeError;

lazy_static! {
    static ref PYLON_MAP: Mutex<HashMap<String, Pylon>> = Mutex::new(HashMap::new());
}

async fn get_pylon() -> Result<Pylon, ThreadSafeError> {
    Pylon::new(Mode::Sender, None).await
}

/// Generates a wormhole code.
/// The newly created FutureConn will be pushed into a global Pylon map to be re-used later.
pub async fn gen_code() -> Result<String, ThreadSafeError> {
    let pylon = get_pylon().await?;
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
pub async fn send_payload(mut payload: Payload) -> Result<Payload, ThreadSafeError> {
    let mut pylon_map = PYLON_MAP.lock().await;
    let pylon = pylon_map.remove(&payload.code);

    if let Some(pylon) = pylon {
        payload.time = Some(SystemTime::now());

        if let Some(message) = &payload.message {
            payload.length = Some(Graphemes::new(message).count());
            payload.checksum = Some(digest(message));
        }

        pylon.activate(Some(&payload)).await?;
    }

    Ok(payload)
}

/// Receives a payload through an encrypted wormhole tunnel.
///
/// # Arguments
///
/// * `code` - The wormhole code to use for PAKE authentication.
pub async fn receive_payload(code: String) -> Result<Payload, ThreadSafeError> {
    let pylon = Pylon::new(Mode::Receiver, Some(code)).await?;
    let payload = pylon.activate(None).await?;

    if let Some(payload) = payload {
        Ok(payload)
    } else {
        Err(Box::new(PylonError("Received empty payload".into())))
    }
}
