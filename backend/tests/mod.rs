//! Unit tests.

#[cfg(test)]
mod tests {
    use std::error::Error;

    use pylon_web::core::{Mode, Payload, Pylon};

    /// Tests whether the Pylon can generate a code when run in Sender mode.
    #[tokio::test]
    async fn test_code_gen() -> Result<(), Box<dyn Error>> {
        let pylon = Pylon::new(Mode::Sender, None).await?;

        if pylon.code.is_none() {
            return Err("Code generation failed".into());
        }

        Ok(())
    }

    /// Tests whether the sent payload isn't corrupted during transit.
    ///
    /// NOTE: The following code may be hideous to look at. It was hacked together in about half an hour
    /// under non-sober conditions, and I couldn't be bothered to properly learn async programming. Viewer
    /// discretion advised.
    #[tokio::test]
    async fn test_payload_match() -> Result<(), Box<dyn Error>> {
        use futures::lock::Mutex;
        use std::sync::Arc;
        use std::time::SystemTime;

        let sender = Pylon::new(Mode::Sender, None).await?;
        let code = sender.code.clone(); //? Is this clone needed?
        let msg = "Hello world";
        let sender_payload: Arc<Mutex<Payload>>;

        if let Some(code) = code {
            sender_payload = Arc::new(Mutex::new(Payload {
                message: Some(msg.into()),
                size: Some(msg.len()),
                time: Some(SystemTime::now()),
                code: code.clone(),
            }));

            //? Is this copy really needed?
            let payload_copy = Arc::clone(&sender_payload);

            // Here comes the ugly part.
            let send_handle = tokio::spawn(async move {
                let sender_payload = Arc::clone(&sender_payload);
                let sender_payload = sender_payload.lock().await;

                sender.activate(Some(&sender_payload)).await.unwrap();
            });
            let recv_handle = tokio::spawn(async move {
                let receiver = Pylon::new(Mode::Receiver, Some(code.clone()))
                    .await
                    .unwrap();
                let sender_payload = Arc::clone(&payload_copy);
                let sender_payload = sender_payload.lock().await;
                let receiver_payload = match receiver.activate(None).await.unwrap() {
                    Some(payload) => payload,
                    None => Payload::default(),
                };

                assert_eq!(*sender_payload, receiver_payload);
            });

            send_handle.await.unwrap(); // Run the payload sender thread.
            recv_handle.await.unwrap(); // Run the payload receiver thread.
        } else {
            // Ideally, this code should be unreachable.
            return Err("Code generation failed".into());
        }

        Ok(())
    }
}
