use reqwest::Method;

use crate::{models::User, utils::request_builder, ClientClient, Result};

/// List, create, update or delete users.
impl ClientClient {
    /// Return the current user.
    pub async fn get_current_user(&self) -> Result<User> {
        self.request(Method::GET, ["current", "user"])
            .send_and_read_json()
            .await
    }
    /// Update the password of the current user.
    pub fn update_current_user(&self, pass: impl Into<String>) -> UpdateCurrentUserBuilder {
        UpdateCurrentUserBuilder::new(self, pass)
    }
    /// Return all users.
    pub async fn get_users(&self) -> Result<Vec<User>> {
        self.request(Method::GET, ["user"])
            .send_and_read_json()
            .await
    }
    /// Create a user.
    pub fn create_user(
        &self,
        admin: bool,
        name: impl Into<String>,
        pass: impl Into<String>,
    ) -> CreateUserBuilder {
        CreateUserBuilder::new(self, admin, name, pass)
    }
    /// Get a user.
    pub async fn get_user(&self, id: i64) -> Result<User> {
        self.request(Method::GET, ["user".into(), id.to_string()])
            .send_and_read_json()
            .await
    }
    /// Update a client.
    pub fn update_user(&self, id: i64, admin: bool, name: impl Into<String>) -> UpdateUserBuilder {
        UpdateUserBuilder::new(self, id, admin, name)
    }
    /// Delete a user.
    pub async fn delete_user(&self, id: i64) -> Result<()> {
        self.request(Method::DELETE, ["user".into(), id.to_string()])
            .send()
            .await
    }
}

request_builder! {
    name = UpdateCurrentUserBuilder,
    client_type = ClientClient,
    method = Method::POST,
    uri = ["current", "user", "password"],
    return_type = (),
    required_fields = {
        pass: impl Into<String> => .into() => String,
    },
    optional_fields = {}
}
request_builder! {
    name = CreateUserBuilder,
    client_type = ClientClient,
    method = Method::POST,
    uri = ["user"],
    return_type = User,
    required_fields = {
        admin: bool => bool,
        name: impl Into<String> => .into() => String,
        pass: impl Into<String> => .into() => String,
    },
    optional_fields = {}
}
request_builder! {
    name = UpdateUserBuilder,
    client_type = ClientClient,
    method = Method::POST,
    uri_with = |builder: &Self| ["user".into(), builder.id.to_string()],
    return_type = User,
    required_fields = {
        #[serde(skip)]
        id: i64 => i64,
        admin: bool => bool,
        name: impl Into<String> => .into() => String,
    },
    optional_fields = {
        pass: impl Into<String> => .into() => String,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::testsuite::*;

    #[apply(run_test_server!)]
    #[test]
    async fn get_current_user() -> eyre::Result<()> {
        let user = client_client().get_current_user().await?;

        assert_eq!(user.name, "admin");

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn update_current_user() -> eyre::Result<()> {
        client_client().update_current_user("new-password").await?;

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn get_users() -> eyre::Result<()> {
        let users = client_client().get_users().await?;

        assert_eq!(
            users.into_iter().map(|a| a.name).collect::<Vec<_>>(),
            vec!["admin"]
        );

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn create_user() -> eyre::Result<()> {
        let client = client_client();

        let user = client.create_user(false, "new-user", "password").await?;
        assert!(!user.admin);
        assert_eq!(user.name, "new-user");

        assert!(client
            .get_users()
            .await?
            .into_iter()
            .find(|u| u.id == user.id)
            .is_some_and(|u| u.name == user.name));

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn get_user() -> eyre::Result<()> {
        let client = client_client();

        let user = client.get_user(1).await?;
        assert_eq!(user.name, "admin");

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn update_user() -> eyre::Result<()> {
        let client = client_client();

        let user = client.create_user(false, "new-user", "password").await?;
        assert_eq!(user.name, "new-user");

        let updated_user = client.update_user(user.id, false, "updated-user").await?;
        assert_eq!(user.id, updated_user.id);
        assert_eq!(updated_user.name, "updated-user");

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn delete_user() -> eyre::Result<()> {
        let client = client_client();

        let user = client.create_user(false, "new-user", "password").await?;
        assert_eq!(user.name, "new-user");

        client.delete_user(user.id).await?;

        assert!(matches!(
            client.get_user(user.id).await,
            Err(crate::Error::Response(crate::models::Error {
                error_code: 404,
                ..
            }))
        ));

        Ok(())
    }
}
