use futures_util::{Stream, StreamExt};
use reqwest::{header, StatusCode};
use tokio_tungstenite::{
    tungstenite::{self, handshake::derive_accept_key},
    WebSocketStream,
};

use crate::{models::Message, utils::UrlAppend, ClientClient};

/// Subscribe to newly created messages.
impl ClientClient {
    /// Return newly created messages via a websocket.
    pub async fn stream_messages(
        &self,
    ) -> Result<impl Stream<Item = Result<Message, WebsocketError>> + '_, WebsocketConnectError>
    {
        // See https://developer.mozilla.org/en-US/docs/Web/HTTP/Protocol_upgrade_mechanism
        let request_key = tungstenite::handshake::client::generate_key();

        let response = self
            .http
            .get(self.base_url.append(["stream"]))
            .version(reqwest::Version::HTTP_11)
            .header(header::CONNECTION, "Upgrade")
            .header(header::UPGRADE, "websocket")
            .header(header::SEC_WEBSOCKET_VERSION, 13)
            .header(header::SEC_WEBSOCKET_KEY, &request_key)
            .header(
                header::SEC_WEBSOCKET_EXTENSIONS,
                "permessage-deflate; client_max_window_bits",
            )
            .send()
            .await?
            .error_for_status()?;

        if response.status() != StatusCode::SWITCHING_PROTOCOLS
            || !response
                .headers()
                .get(header::SEC_WEBSOCKET_ACCEPT)
                .and_then(|v| v.to_str().ok())
                .is_some_and(|key| key == derive_accept_key(request_key.as_ref()))
        {
            return Err(WebsocketConnectError::Response(response));
        }

        let mut ws = WebSocketStream::from_raw_socket(
            response
                .upgrade()
                .await
                .map_err(WebsocketConnectError::Upgrade)?,
            tungstenite::protocol::Role::Client,
            None,
        )
        .await;

        let stream = async_stream::stream! {
            while let Some(res) = ws.next().await {
                match res {
                    Ok(tungstenite::Message::Text(msg)) => {
                        yield serde_json::from_str(&msg).map_err(WebsocketError::Serde)
                    }
                    Ok(_) => continue,
                    Err(e) => yield Err(e.into()),
                }
            }

        };

        Ok(Box::pin(stream))
    }
}

/// Errors that can occur when initializing the websocket connection.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum WebsocketConnectError {
    #[error("initial HTTP request failed")]
    Http(#[from] reqwest::Error),
    #[error("server did not return a valid upgradable response: {0:?}")]
    Response(reqwest::Response),
    #[error("connection upgrade failed")]
    Upgrade(#[source] reqwest::Error),
    #[error("a websocket error occured")]
    Websocket(#[from] tungstenite::Error),
}

/// Errors that can occur when the websocket is established.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum WebsocketError {
    #[error("a websocket error occured")]
    Websocket(#[from] tungstenite::Error),
    #[error("failed to deserialize message")]
    Serde(#[from] serde_json::Error),
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::testsuite::*;

    #[apply(run_test_server!)]
    #[test]
    async fn stream_messages() -> eyre::Result<()> {
        use futures_util::StreamExt;

        let app_client = app_client();
        let client_client = client_client();

        let mut stream = client_client.stream_messages().await?;

        for i in 1..=10 {
            let msg = format!("message-{i}");

            app_client.create_message(&msg).await?;

            assert_eq!(stream.next().await.unwrap().unwrap().message, msg);
        }

        Ok(())
    }
}
