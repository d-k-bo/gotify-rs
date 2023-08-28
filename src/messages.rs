use reqwest::Method;

use crate::{models::PagedMessages, utils::request_builder, ClientClient, Result};

/// List or delete messages.
impl ClientClient {
    /// Return all messages from a specific application.
    pub fn get_application_messages(&self, id: i64) -> GetApplicationMessagesBuilder {
        GetApplicationMessagesBuilder::new(self, id)
    }
    /// Delete all messages from a specific application.
    pub async fn delete_application_messages(&self, id: i64) -> Result<()> {
        self.request(
            Method::DELETE,
            ["application".into(), id.to_string(), "message".into()],
        )
        .send()
        .await
    }
    /// Return all messages.
    pub fn get_messages(&self) -> GetMessagesBuilder {
        GetMessagesBuilder::new(self)
    }
    /// Delete all messages.
    pub async fn delete_messages(&self) -> Result<()> {
        self.request(Method::DELETE, ["message"]).send().await
    }
    /// Delete a message with an id.
    pub async fn delete_message(&self, id: i64) -> Result<()> {
        self.request(Method::DELETE, ["message".into(), id.to_string()])
            .send()
            .await
    }
}

request_builder! {
    name = GetApplicationMessagesBuilder,
    client_type = ClientClient,
    method = Method::GET,
    uri_with = |builder: &Self| ["application".into(), builder.id.to_string(), "message".into()],
    return_type = PagedMessages,
    required_fields = {
        #[serde(skip)]
        id: i64 => i64,
    },
    optional_fields = {
        limit: usize => usize,
        since: i64 => i64,
    }
}
request_builder! {
    name = GetMessagesBuilder,
    client_type = ClientClient,
    method = Method::GET,
    uri = ["message"],
    return_type = PagedMessages,
    required_fields = {},
    optional_fields = {
        limit: usize => usize,
        since: i64 => i64,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::testsuite::*;

    #[apply(run_test_server!)]
    #[test]
    async fn get_application_messages() -> eyre::Result<()> {
        let client = client_client();

        let messages = client.get_application_messages(3).await?;

        assert_eq!(
            messages
                .messages
                .iter()
                .map(|m| &m.message)
                .collect::<Vec<_>>(),
            vec!["App1-Message1", "App1-Message0"]
        );

        let messages = client.get_application_messages(3).with_limit(1).await?;

        assert_eq!(
            messages
                .messages
                .iter()
                .map(|m| &m.message)
                .collect::<Vec<_>>(),
            vec!["App1-Message1"]
        );

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn delete_application_messages() -> eyre::Result<()> {
        let client = client_client();

        assert!(!client
            .get_application_messages(3)
            .await?
            .messages
            .is_empty());

        client.delete_application_messages(3).await?;

        assert!(client
            .get_application_messages(3)
            .await?
            .messages
            .is_empty());

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn get_messages() -> eyre::Result<()> {
        let client = client_client();

        let messages = client.get_messages().with_limit(3).await?;
        assert_eq!(messages.messages.len(), 3);

        let messages = client.get_messages().with_since(5).await?;
        assert_eq!(messages.messages.len(), 4);

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn delete_messages() -> eyre::Result<()> {
        let client = client_client();

        assert!(!client.get_messages().await?.messages.is_empty());

        client.delete_messages().await?;

        assert!(client.get_messages().await?.messages.is_empty());

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn delete_message() -> eyre::Result<()> {
        let client = client_client();

        assert!(client
            .get_messages()
            .await?
            .messages
            .iter()
            .any(|m| m.id == 1));

        client.delete_message(1).await?;

        assert!(!client
            .get_messages()
            .await?
            .messages
            .iter()
            .any(|m| m.id == 1));

        Ok(())
    }
}
