use reqwest::Method;

use crate::{models::VersionInfo, Client, Result};

impl<T> Client<T> {
    /// Get version information.
    pub async fn version(&self) -> Result<VersionInfo> {
        self.request(Method::GET, ["version"]).await
    }
}

#[cfg(test)]
mod tests {
    use crate::testsuite::*;

    #[apply(run_test_server!)]
    #[test]
    async fn version() -> eyre::Result<()> {
        unauthenticated_client().version().await?;

        Ok(())
    }
}
