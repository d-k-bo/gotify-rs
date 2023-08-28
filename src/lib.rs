//! An idiomatic Rust client for Gotify.
//!
//! ## Overview
//!
//! By default, this crate only exposes the [`Client::health()`](crate::Client::health), [`Client::version()`](crate::Client::version) methods.
//! All other categories of endpoints must be enabled by the correspondig feature flags.
//!
//! | Feature flag | Enabled methods | Note |
//! | ------------ | --------------- | ---- |
//! | `app` | [`Client::create_message()`](crate::Client::create_message) | |
//! | `manage-clients` | [`Client::get_clients()`](crate::Client::get_clients), [`Client::create_client()`](crate::Client::create_client), [`Client::update_client()`](crate::Client::update_client), [`Client::delete_client()`](crate::Client::delete_client) | |
//! | `manage-messages` | [`Client::get_application_messages()`](crate::Client::get_application_messages), [`Client::delete_application_messages()`](crate::Client::delete_application_messages), [`Client::get_messages()`](crate::Client::get_messages), [`Client::delete_messages()`](crate::Client::delete_messages), [`Client::delete_message()`](crate::Client::delete_message) | doesn't include [`Client::create_message()`](crate::Client::create_message) and [`Client::message_stream()`](crate::Client::message_stream) |
//! | `manage-plugins` | [`Client::get_plugins()`](crate::Client::get_plugins), [`Client::get_plugin_config()`](crate::Client::get_plugin_config), [`Client::update_plugin_config()`](crate::Client::update_plugin_config), [`Client::disable_plugin()`](crate::Client::disable_plugin), [`Client::get_plugin_display()`](crate::Client::get_plugin_display), [`Client::enable_plugin()`](crate::Client::enable_plugin) | |
//! | `websocket` | [`Client::message_stream()`](crate::Client::message_stream) | enables additional dependencies (mainly [`tokio-tungstenite`](https://docs.rs/tokio-tungstenite)) |
//!
//! ## Examples
//!
//! ### Creating a message
//!
//! ```ignore
//! let client: gotify::AppClient = gotify::Client::new(GOTIFY_URL, GOTIFY_APP_TOKEN)?;
//!
//! client.create_message("Lorem ipsum dolor sit amet").with_title("Lorem Ipsum").await?;
//! ```
//!
//! ### Listening for new messages
//!
//! ```ignore
//! use futures_util::StreamExt;
//!
//! let client: gotify::ClientClient = gotify::Client::new(GOTIFY_URL, GOTIFY_CLIENT_TOKEN)?;
//!
//! let mut messages = client.message_stream().await?;
//!
//! while let Some(result) = messages.next().await {
//!     let message = result?;
//!
//!     println!("{message:#?}")
//! }
//! ```

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

use std::marker::PhantomData;

use reqwest::{
    header::{HeaderMap, HeaderValue, InvalidHeaderValue},
    Method,
};
use url::Url;

use crate::utils::UrlAppend;

pub use crate::error::{Error, InitError, Result};
#[cfg(feature = "websocket")]
#[cfg_attr(docsrs, doc(cfg(feature = "websocket")))]
pub use crate::websocket::{WebsocketConnectError, WebsocketError};

pub mod models;

/// Builder structs used by some methods that send data to Gotify's API.
///
/// While they provide a `send()` method, they also implement
/// [`IntoFuture`](std::future::IntoFuture) and can be `await`ed directly.
pub mod builder {
    #[cfg(feature = "app")]
    #[cfg_attr(docsrs, doc(cfg(feature = "app")))]
    pub use crate::app::MessageBuilder;
    #[cfg(feature = "manage-applications")]
    #[cfg_attr(docsrs, doc(cfg(feature = "manage-applications")))]
    pub use crate::applications::{ApplicationBuilder, ApplicationUpdateBuilder};
    #[cfg(feature = "manage-clients")]
    #[cfg_attr(docsrs, doc(cfg(feature = "manage-clients")))]
    pub use crate::clients::{ClientBuilder, ClientUpdateBuilder};
    #[cfg(feature = "manage-messages")]
    #[cfg_attr(docsrs, doc(cfg(feature = "manage-messages")))]
    pub use crate::messages::{GetApplicationMessagesBuilder, GetMessagesBuilder};
    #[cfg(feature = "manage-users")]
    #[cfg_attr(docsrs, doc(cfg(feature = "manage-users")))]
    pub use crate::users::{CreateUserBuilder, UpdateCurrentUserBuilder, UpdateUserBuilder};
}

#[cfg(feature = "app")]
#[cfg_attr(docsrs, doc(cfg(feature = "app")))]
mod app;
#[cfg(feature = "manage-applications")]
#[cfg_attr(docsrs, doc(cfg(feature = "manage-applications")))]
mod applications;
#[cfg(feature = "manage-clients")]
#[cfg_attr(docsrs, doc(cfg(feature = "manage-clients")))]
mod clients;
mod error;
mod health;
#[cfg(feature = "manage-messages")]
#[cfg_attr(docsrs, doc(cfg(feature = "manage-messages")))]
mod messages;
#[cfg(feature = "manage-plugins")]
#[cfg_attr(docsrs, doc(cfg(feature = "manage-plugins")))]
mod plugins;
#[cfg(feature = "manage-users")]
#[cfg_attr(docsrs, doc(cfg(feature = "manage-users")))]
mod users;
mod version;
#[cfg(feature = "websocket")]
#[cfg_attr(docsrs, doc(cfg(feature = "websocket")))]
mod websocket;

#[cfg(test)]
pub(crate) mod testsuite;
mod utils;

/// A client for a specific Gotify server. The main entrypoint of this crate.
///
/// It comes in three varieties to perform different tasks.
///
/// | Type | Explanation | Feature flag |
/// | ---- | ------------ | ----------------- |
/// | [`UnauthenticatedClient = Client<Unauthenticated>`](crate::UnauthenticatedClient) | get server status and version info | always enabled |
/// | [`AppClient = Client<AppToken>`](crate::AppClient) | create messages | `app` |
/// | [`ClientClient = Client<ClientToken>`](crate::ClientClient) | manage the server (anything else) | any of `manage-*` or `websocket` |
#[derive(Clone, Debug)]
pub struct Client<T> {
    base_url: Url,
    http: reqwest::Client,
    token: PhantomData<T>,
}

/// A client that is authenticated to create messages.
#[cfg(feature = "app")]
#[cfg_attr(docsrs, doc(cfg(feature = "app")))]
pub type AppClient = Client<AppToken>;

/// A client that is authenticated to manage the server.
#[cfg(feature = "client-core")]
#[cfg_attr(docsrs, doc(cfg(feature = "client-core")))]
pub type ClientClient = Client<ClientToken>;

/// A client that is unauthenticated.
pub type UnauthenticatedClient = Client<Unauthenticated>;

/// Marks a client as authenticated to create messages.
#[cfg(feature = "app")]
#[cfg_attr(docsrs, doc(cfg(feature = "app")))]
#[derive(Clone, Debug)]
pub struct AppToken;

/// Marks a client as authenticated to manage the server.
#[cfg(feature = "client-core")]
#[cfg_attr(docsrs, doc(cfg(feature = "client-core")))]
#[derive(Clone, Debug)]
pub struct ClientToken;

/// Marks a client as unauthenticated.
#[derive(Clone, Debug)]
pub struct Unauthenticated;

/// Sealed trait to represent an [`AppToken`] or [`ClientToken`].
pub trait TokenType: private::Sealed {}

#[cfg(feature = "app")]
#[cfg_attr(docsrs, doc(cfg(feature = "app")))]
impl TokenType for AppToken {}

#[cfg(feature = "client-core")]
#[cfg_attr(docsrs, doc(cfg(feature = "client-core")))]
impl TokenType for ClientToken {}

mod private {
    pub trait Sealed {}

    #[cfg(feature = "app")]
    #[cfg_attr(docsrs, doc(cfg(feature = "app")))]
    impl Sealed for super::AppToken {}

    #[cfg(feature = "client-core")]
    #[cfg_attr(docsrs, doc(cfg(feature = "client-core")))]
    impl Sealed for super::ClientToken {}
}

#[cfg(any(feature = "app", feature = "client-core"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "app", feature = "client-core"))))]
impl<T: TokenType> Client<T> {
    /// Create a new authenticated client.
    ///
    /// The type of the used access token (app token or client token)
    /// must be provided as a generic parameter or be inferable.
    pub fn new(
        server_url: impl TryInto<Url, Error = url::ParseError>,
        access_token: impl TryInto<HeaderValue, Error = InvalidHeaderValue>,
    ) -> core::result::Result<Self, InitError> {
        Ok(Client {
            base_url: server_url.try_into()?,
            http: reqwest::Client::builder()
                .default_headers({
                    let mut headers = HeaderMap::new();
                    headers.insert("X-Gotify-Key", access_token.try_into()?);
                    headers
                })
                .build()
                .map_err(InitError::Reqwest)?,
            token: PhantomData,
        })
    }
}

impl Client<Unauthenticated> {
    /// Create a new unauthenticated client.
    ///
    /// This type by itself has very limited capabilities but can be authenticated later on.
    pub fn new_unauthenticated(
        server_url: impl TryInto<Url, Error = url::ParseError>,
    ) -> core::result::Result<Self, InitError> {
        Ok(Client {
            base_url: server_url.try_into()?,
            http: reqwest::Client::new(),
            token: PhantomData,
        })
    }

    /// Create an authenticated client from this unauthenicated client.
    ///
    /// The type of the used access token (app token or client token)
    /// must be provided as a generic parameter or be inferable.
    pub fn authenticate<T: TokenType>(
        self,
        access_token: impl TryInto<HeaderValue, Error = InvalidHeaderValue>,
    ) -> core::result::Result<Client<T>, InitError> {
        Ok(Client {
            base_url: self.base_url,
            http: reqwest::Client::builder()
                .default_headers({
                    let mut headers = HeaderMap::new();
                    headers.insert("X-Gotify-Key", access_token.try_into()?);
                    headers
                })
                .build()
                .map_err(InitError::Reqwest)?,
            token: PhantomData,
        })
    }
}

pub(crate) struct RequestBuilder(reqwest::RequestBuilder);
impl RequestBuilder {
    #[cfg(any(feature = "app", feature = "client-core"))]
    pub fn with_query(self, params: impl serde::Serialize) -> Self {
        Self(self.0.query(&params))
    }
    #[cfg(any(feature = "app", feature = "client-core"))]
    pub fn with_json_body(self, body: impl serde::Serialize) -> Self {
        Self(self.0.json(&body))
    }
    #[cfg(feature = "manage-plugins")]
    pub fn with_string_body(self, body: String) -> Self {
        Self(self.0.body(body))
    }
    #[cfg(feature = "manage-messages")]
    pub fn with_file(
        self,
        file_name: impl Into<std::borrow::Cow<'static, str>>,
        file_content: impl Into<std::borrow::Cow<'static, [u8]>>,
    ) -> Self {
        use reqwest::multipart::{Form, Part};

        Self(
            self.0.multipart(
                Form::new().part("file", Part::bytes(file_content).file_name(file_name)),
            ),
        )
    }
}
impl RequestBuilder {
    #[cfg(feature = "client-core")]
    pub async fn send(self) -> Result<()> {
        let r = self.0.send().await?;

        if r.status().is_success() {
            Ok(())
        } else {
            Err(Error::Response(r.json().await?))
        }
    }
    pub async fn send_and_read_json<R: for<'a> serde::Deserialize<'a> + 'static>(
        self,
    ) -> Result<R> {
        let r = self.0.send().await?;

        if r.status().is_success() {
            Ok(r.json().await?)
        } else {
            Err(Error::Response(r.json().await?))
        }
    }
    #[cfg(feature = "manage-plugins")]
    pub async fn send_and_read_string(self) -> Result<String> {
        let r = self.0.send().await?;

        if r.status().is_success() {
            Ok(r.text().await?)
        } else {
            Err(Error::Response(r.json().await?))
        }
    }
}

impl<T> Client<T> {
    pub(crate) fn request(
        &self,
        method: Method,
        uri: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> RequestBuilder {
        RequestBuilder(self.http.request(method, self.base_url.append(uri)))
    }
}

#[cfg(test)]
mod tests {
    use crate::testsuite::*;

    #[apply(run_test_server!)]
    #[test]
    async fn authenticate() -> eyre::Result<()> {
        use crate::{AppToken, ClientToken};

        let client = unauthenticated_client();

        let app_client = client
            .as_ref()
            .clone()
            .authenticate::<AppToken>(GOTIFY_APP_TOKEN)?;
        let client_client = client
            .as_ref()
            .clone()
            .authenticate::<ClientToken>(GOTIFY_CLIENT_TOKEN)?;

        app_client.create_message("foobar").await?;
        client_client.get_messages().await?;

        Ok(())
    }
}
