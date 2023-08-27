use reqwest::header::InvalidHeaderValue;

/// Errors that can occur when creating or authenticating a [`Client`](crate::Client).
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("could not parse the server URL")]
    InvalidUrl(#[from] url::ParseError),
    #[error("invalid access token")]
    InvalidAccessToken(#[from] InvalidHeaderValue),
    #[error("failed to initialize the HTTP client")]
    Reqwest(#[from] reqwest::Error),
}

/// Errors that can occur when accessing an API endpoint.
#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed")]
    Reqwest(#[from] reqwest::Error),
    #[error("Gotify's API returned an error")]
    Response(#[from] crate::models::Error),
}

/// Alias for the `Result` returned when accessing an API endpoint.
pub type Result<T> = core::result::Result<T, Error>;
