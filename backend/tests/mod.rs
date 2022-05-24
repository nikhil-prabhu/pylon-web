//! Unit tests.

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::sync::Arc;

    use pylon_web::core::{Mode, Payload, Pylon};

    use futures::lock::Mutex;

    use unic_segment::Graphemes;

    use sha256::digest;

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
        let sender = Pylon::new(Mode::Sender, None).await?;
        let code = sender.code.clone(); //? Is this clone needed?
        let msg = "Hello world";
        let sender_payload: Arc<Mutex<Payload>>;

        if let Some(code) = code {
            sender_payload = Arc::new(Mutex::new(Payload::from((msg, code.clone().as_str()))));

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

    /// Tests if a Payload can be created from a (&str, &str).
    #[test]
    fn test_payload_from() {
        let msg = "Hello world";
        let code = "1-hello-world";

        // The `time` field will not be equal, since the exact creation time of the payload and derived payloads
        // will vary. Hence, we set it to `None`, and only assert that the other fields are equal.
        let payload = Payload {
            message: Some(msg.into()),
            length: Some(Graphemes::new(msg).count()),
            code: code.into(),
            time: None,
            checksum: Some(digest(msg)),
        };
        let derived_payload = Payload::from((msg, code));

        assert_eq!(payload.message, derived_payload.message);
        assert_eq!(payload.code, derived_payload.code);
        assert_eq!(payload.length, derived_payload.length);
    }

    /// Tests whether a (&str, &str) can be converted to a Payload.
    #[test]
    fn test_tuple_into() {
        let msg = "Hello world";
        let code = "1-hello-world";

        // The `time` field will not be equal, since the exact creation time of the payload and derived payloads
        // will vary. Hence, we set it to `None`, and only assert that the other fields are equal.
        let payload = Payload {
            message: Some(msg.into()),
            length: Some(Graphemes::new(msg).count()),
            code: code.into(),
            time: None,
            checksum: Some(digest(msg)),
        };
        let derived_payload: Payload = (msg, code).into();

        assert_eq!(payload.message, derived_payload.message);
        assert_eq!(payload.code, derived_payload.code);
        assert_eq!(payload.length, derived_payload.length);
    }
}
