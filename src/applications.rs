use std::borrow::Cow;

use reqwest::Method;

use crate::{models::Application, utils::request_builder, ClientClient, Result};

/// Create, read, update and delete applications or modify application images.
impl ClientClient {
    /// Return all applications.
    pub async fn get_applications(&self) -> Result<Vec<Application>> {
        self.request(Method::GET, ["application"])
            .send_and_read_json()
            .await
    }
    /// Create an application.
    pub fn create_application(&self, name: impl Into<String>) -> ApplicationBuilder {
        ApplicationBuilder::new(self, name)
    }
    /// Update an application.
    pub fn update_application(&self, id: i64, name: impl Into<String>) -> ApplicationUpdateBuilder {
        ApplicationUpdateBuilder::new(self, id, name)
    }
    /// Delete an application.
    pub async fn delete_application(&self, id: i64) -> Result<()> {
        self.request(Method::DELETE, ["application".into(), id.to_string()])
            .send()
            .await
    }
    /// Upload an image for an application.
    pub async fn upload_application_image(
        &self,
        id: i64,
        image_name: impl Into<Cow<'static, str>>,
        image_content: impl Into<Cow<'static, [u8]>>,
    ) -> Result<Application> {
        self.request(
            Method::POST,
            ["application".into(), id.to_string(), "image".into()],
        )
        .with_file(image_name, image_content)
        .send_and_read_json()
        .await
    }
    /// Delete an image of an application.
    pub async fn delete_application_image(&self, id: i64) -> Result<()> {
        self.request(
            Method::DELETE,
            ["application".into(), id.to_string(), "image".into()],
        )
        .send()
        .await
    }
}

request_builder! {
    name = ApplicationBuilder,
    client_type = ClientClient,
    method = Method::POST,
    uri = ["application"],
    return_type = Application,
    required_fields = {
        name: impl Into<String> => .into() => String,
    },
    optional_fields = {
        default_priority: u8 => u8,
        description: impl Into<String> => .into() => String,
    }
}
request_builder! {
    name = ApplicationUpdateBuilder,
    client_type = ClientClient,
    method = Method::PUT,
    uri_with = |builder: &Self| ["application".into(), builder.id.to_string()],
    return_type = Application,
    required_fields = {
        #[serde(skip)]
        id: i64 => i64,
        name: impl Into<String> => .into() => String,
    },
    optional_fields = {
        default_priority: u8 => u8,
        description: impl Into<String> => .into() => String,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::testsuite::*;

    #[apply(run_test_server!)]
    #[test]
    async fn get_applications() -> eyre::Result<()> {
        let applications = client_client().get_applications().await?;

        assert_eq!(
            applications
                .into_iter()
                .take(3)
                .map(|a| a.name)
                .collect::<Vec<_>>(),
            vec!["gotify-rs", "App0", "App1"]
        );

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn create_application() -> eyre::Result<()> {
        let client = client_client();

        let application = client.create_application("new-application1").await?;
        assert_eq!(application.name, "new-application1");
        assert_eq!(application.description, "");

        let application = client
            .create_application("new-application2")
            .with_description("application with a description")
            .await?;
        assert_eq!(application.name, "new-application2");
        assert_eq!(application.description, "application with a description");

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn update_application() -> eyre::Result<()> {
        let client = client_client();

        let application = client.create_application("new-application").await?;
        assert_eq!(application.name, "new-application");
        assert_eq!(application.description, "");

        let application = client
            .update_application(application.id, "updated-application")
            .with_description("updated application")
            .await?;
        assert_eq!(application.name, "updated-application");
        assert_eq!(application.description, "updated application");

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn delete_application() -> eyre::Result<()> {
        let client = client_client();

        assert!(client.get_applications().await?.iter().any(|a| a.id == 1));

        client.delete_application(1).await?;

        assert!(!client.get_applications().await?.iter().any(|a| a.id == 1));

        Ok(())
    }

    #[apply(run_test_server!)]
    #[test]
    async fn update_application_image() -> eyre::Result<()> {
        let client = client_client();

        assert!(client
            .get_applications()
            .await?
            .iter()
            .find(|a| a.id == 1)
            .is_some_and(|a| a.image == "static/defaultapp.png"));

        let application = client
            .upload_application_image(1, "img.png", include_bytes!("../tests/img.png").as_slice())
            .await?;

        assert!(application.image.starts_with("image/"));

        Ok(())
    }
}
