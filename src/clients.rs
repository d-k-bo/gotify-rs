use reqwest::Method;

use crate::{models::Client, utils::request_builder, ClientClient, Result};

/// List, create, update or delete clients.
impl ClientClient {
    /// Return all clients.
    pub async fn get_clients(&self) -> Result<Vec<Client>> {
        self.request(Method::GET, ["client"]).await
    }
    /// Create a client.
    pub fn create_client(&self, name: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(self, name)
    }
    /// Update a client.
    pub fn update_client(&self, id: i64, name: impl Into<String>) -> ClientUpdateBuilder {
        ClientUpdateBuilder::new(self, id, name)
    }
    /// Delete a client.
    pub async fn delete_client(&self, id: i64) -> Result<()> {
        self.request(Method::DELETE, ["client".into(), id.to_string()])
            .await
    }
}

request_builder! {
    name = ClientBuilder,
    client_type = ClientClient,
    method = Method::POST,
    uri = ["client"],
    return_type = Client,
    required_fields = {
        name: impl Into<String> => .into() => String,
    },
    optional_fields = {}
}
request_builder! {
    name = ClientUpdateBuilder,
    client_type = ClientClient,
    method = Method::PUT,
    uri_with = |builder: &Self| ["client".into(), builder.id.to_string()],
    return_type = Client,
    required_fields = {
        #[serde(skip)]
        id: i64 => i64,
        name: impl Into<String> => .into() => String,
    },
    optional_fields = {}
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::testsuite::*;

    #[apply(run_test_server!)]
    #[test]
    async fn get_clients() -> eyre::Result<()> {
        let clients = client_client().get_clients().await?;

        assert_eq!(
            clients.into_iter().map(|a| a.name).collect::<Vec<_>>(),
            vec!["gotify-rs"]
        );

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn create_client() -> eyre::Result<()> {
        let client = client_client().create_application("new-client").await?;
        assert_eq!(client.name, "new-client");
        assert_eq!(client.description, "");

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn update_client() -> eyre::Result<()> {
        let client = client_client();

        assert!(!client
            .get_clients()
            .await?
            .iter()
            .any(|a| a.name == "new-client-name"));

        let client = client.update_client(1, "new-client-name").await?;

        assert_eq!(client.name, "new-client-name");

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn delete_client() -> eyre::Result<()> {
        let client = client_client();

        let new_client = client.create_client("new-client").await?;

        assert!(client
            .get_clients()
            .await?
            .iter()
            .any(|a| a.id == new_client.id));

        client.delete_client(new_client.id).await?;

        assert!(!client
            .get_clients()
            .await?
            .iter()
            .any(|a| a.id == new_client.id));

        Ok(())
    }
}
