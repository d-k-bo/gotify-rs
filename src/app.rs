use std::collections::HashMap;

use reqwest::Method;

use crate::{models::Message, utils::request_builder, AppClient};

/// Create messages.
impl AppClient {
    /// Create a message.
    pub fn create_message(&self, message: impl Into<String>) -> MessageBuilder {
        MessageBuilder::new(self, message)
    }
}

request_builder! {
    name = MessageBuilder,
    client_type = AppClient,
    method = Method::POST,
    uri = ["message"],
    return_type = Message,
    required_fields = {
        message: impl Into<String> => .into() => String,
    },
    optional_fields = {
        title: impl Into<String> => .into() => String,
        extras: impl Into<HashMap<String, serde_json::Value>> => .into() => HashMap<String, serde_json::Value>,
        priority: u8 => u8,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::testsuite::*;

    #[apply(run_test_server!)]
    #[test]
    async fn create_message() -> eyre::Result<()> {
        let client = app_client();

        let message = client.create_message("Hello World").await?;
        assert_eq!(message.message, "Hello World");

        let message = client
            .create_message("Hello World")
            .with_title("Hi")
            .with_priority(7)
            .with_extras([("foo".into(), "bar".into())])
            .await?;
        assert_eq!(message.title.as_deref(), Some("Hi"));
        assert_eq!(message.message, "Hello World");
        assert_eq!(message.priority, 7);
        assert_eq!(
            message
                .extras
                .unwrap()
                .get("foo")
                .unwrap()
                .as_str()
                .unwrap(),
            "bar"
        );

        Ok(())
    }
}
