use reqwest::Method;

use crate::{models::PluginConf, ClientClient, Result};

/// List or configure plugins.
impl ClientClient {
    /// Return all plugins.
    pub async fn get_plugins(&self) -> Result<Vec<PluginConf>> {
        self.request(Method::GET, ["plugin"])
            .send_and_read_json()
            .await
    }
    /// Get YAML configuration for Configurer plugin.
    pub async fn get_plugin_config(&self, id: i64) -> Result<String> {
        self.request(
            Method::GET,
            ["plugin".into(), id.to_string(), "config".into()],
        )
        .send_and_read_string()
        .await
    }
    /// Update YAML configuration for Configurer plugin.
    pub async fn update_plugin_config(&self, config: String) -> Result<()> {
        self.request(Method::GET, ["user"])
            .with_string_body(config)
            .send()
            .await
    }
    /// Disable a plugin.
    pub async fn disable_plugin(&self, id: i64) -> Result<()> {
        self.request(
            Method::POST,
            ["plugin".into(), id.to_string(), "disable".into()],
        )
        .send()
        .await
    }
    /// Get display info for a Displayer plugin.
    pub async fn get_plugin_display(&self, id: i64) -> Result<String> {
        self.request(
            Method::GET,
            ["plugin".into(), id.to_string(), "display".into()],
        )
        .send_and_read_string()
        .await
    }
    /// Enable a plugin.
    pub async fn enable_plugin(&self, id: i64) -> Result<()> {
        self.request(
            Method::POST,
            ["plugin".into(), id.to_string(), "enable".into()],
        )
        .send()
        .await
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::testsuite::*;

    #[apply(run_test_server!)]
    #[test]
    async fn get_plugins() -> eyre::Result<()> {
        let clients = client_client().get_plugins().await?;

        assert!(clients.is_empty());

        Ok(())
    }
}
