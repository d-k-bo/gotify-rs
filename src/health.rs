use reqwest::Method;

use crate::{models::Health, Client, Result};

impl<T> Client<T> {
    /// Get health information.
    pub async fn health(&self) -> Result<Health> {
        self.request(Method::GET, ["health"]).await
    }
}

#[cfg(test)]
mod tests {
    use crate::testsuite::*;

    #[apply(run_test_server!)]
    #[test]
    async fn health() -> eyre::Result<()> {
        let health = unauthenticated_client().health().await?;
        assert_eq!(health.database, "green");
        assert_eq!(health.health, "green");

        Ok(())
    }
}
