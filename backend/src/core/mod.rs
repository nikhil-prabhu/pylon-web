//! The core message sending/receiving functionality.

use std::borrow::Cow;
use std::error::Error;
use std::fmt;
use std::future::Future;
use std::time::SystemTime;

use magic_wormhole::rendezvous::DEFAULT_RENDEZVOUS_SERVER;
use magic_wormhole::{AppConfig, AppID, Code, Wormhole, WormholeError};

use serde::{Deserialize, Serialize};

use crate::consts::{APP_ID, APP_VERSION, CODE_LENGTH};

/// A connection that hasn't yet been established.
/// It must be awaited to perform the client-client handshake and establish the connection.
type FutureConn = Box<dyn Future<Output = Result<Wormhole, WormholeError>> + Unpin + Send + Sync>;

/// An established wormhole connection.
type Conn = Wormhole;

/// A custom error type for Pylon errors.
#[derive(Debug, Serialize)]
pub struct PylonError(pub String);

impl fmt::Display for PylonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for PylonError {}

/// Represents the message payload.
/// This payload can be sent and received through the encrypted wormhole tunnel.
///
/// # Fields
///
/// * `message` - The message to send (sender mode)/that was received (receiver mode).
/// * `size` - The size/length of the message.
/// * `code`- The wormhole code.
/// * `time` - The time the message was sent.
#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub message: Option<String>,
    pub size: Option<usize>,
    pub code: String,
    pub time: Option<SystemTime>,
}

/// The Pylon mode.
///
/// # Variants
///
/// * `Sender` - Mode used to send messages.
/// * `Receiver` - Mode used to receive messages.
pub enum Mode {
    Sender,
    Receiver,
}

/// The Pylon connection mode (Sender/Receiver).
///
/// # Variants
///
/// * `Sender` - A future sender connection.
/// * `Receiver` - An established receiver connection.
#[allow(clippy::large_enum_variant)]
enum ConnType {
    FutureConn(FutureConn),
    EstConn(Conn),
}

/// An object that can send or receive messages using an encrypted wormhole tunnel.
/// Named after the pylons in Terraria.
///
/// # Fields
///
/// * `conn` - A wormhole connection in sender or receiver mode.
/// * `code` - The generated wormhole code for PAKE authentication (only populated in Sender mode).
pub struct Pylon {
    conn: ConnType,
    pub code: Option<String>,
}

impl Pylon {
    /// Creates a new Pylon in sender or receiver mode.
    ///
    /// # Arguments
    ///
    /// * `mode` - The Pylon mode (Sender/Receiver).
    /// * `code` - The wormhole code for PAKE authentication (only required in Receiver mode).
    pub async fn new(mode: Mode, code: Option<String>) -> Result<Self, Box<dyn Error>> {
        let conf = AppConfig {
            id: AppID(Cow::from(APP_ID)),
            rendezvous_url: Cow::from(DEFAULT_RENDEZVOUS_SERVER),
            app_version: APP_VERSION,
        };

        match mode {
            Mode::Sender => {
                let conn = Wormhole::connect_without_code(conf, CODE_LENGTH)
                    .await
                    .unwrap();
                let code = conn.0.code;

                Ok(Self {
                    conn: ConnType::FutureConn(Box::new(Box::pin(conn.1))),
                    code: Some(code.0),
                })
            }
            Mode::Receiver => {
                if let Some(code) = code {
                    let conn = Wormhole::connect_with_code(conf, Code(code)).await.unwrap();

                    return Ok(Self {
                        conn: ConnType::EstConn(conn.1),
                        code: None,
                    });
                }

                Err(Box::new(PylonError(
                    "Wormhole code is required to establish the connection".into(),
                )))
            }
        }
    }

    /// "Activates" the Pylon, and performs a send or a receive operation.
    ///
    /// # Arguments
    ///
    /// * `payload` - The payload to send (only required in Sender mode).
    pub async fn activate(
        self,
        payload: Option<Payload>,
    ) -> Result<Option<Payload>, Box<dyn Error>> {
        let payload = payload;

        match self.conn {
            ConnType::FutureConn(conn) => {
                if let Some(payload) = payload {
                    let mut wh = conn.await.unwrap();
                    wh.send_json(&payload).await.unwrap();

                    Ok(None)
                } else {
                    Err(Box::new(PylonError(
                        "Payload cannot be empty in Sender mode".into(),
                    )))
                }
            }
            ConnType::EstConn(mut conn) => {
                let payload = conn.receive_json().await.unwrap().unwrap();

                Ok(Some(payload))
            }
        }
    }
}
