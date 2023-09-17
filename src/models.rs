//! JSON models returned by Gotify's API.
#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

#[cfg(feature = "manage-applications")]
#[cfg_attr(docsrs, doc(cfg(feature = "manage-applications")))]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Application {
    pub default_priority: Option<u8>,
    pub description: String,
    pub id: i64,
    pub image: String,
    pub internal: bool,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_used: Option<time::OffsetDateTime>,
    pub name: String,
    pub token: String,
}

#[cfg(feature = "manage-clients")]
#[cfg_attr(docsrs, doc(cfg(feature = "manage-clients")))]
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Client {
    pub id: i64,
    #[serde(default, with = "time::serde::iso8601::option")]
    pub last_used: Option<time::OffsetDateTime>,
    pub name: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Error {
    pub error: String,
    pub error_code: u16,
    pub error_description: String,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}: {}",
            self.error_code, self.error, self.error_description
        )
    }
}
impl std::error::Error for Error {}

#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Health {
    pub database: String,
    pub health: String,
}

#[cfg(any(feature = "app", feature = "manage-messages", feature = "websocket"))]
#[cfg_attr(
    docsrs,
    doc(cfg(any(feature = "app", feature = "manage-messages", feature = "websocket")))
)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Message {
    pub appid: i64,
    #[serde(with = "time::serde::iso8601")]
    pub date: time::OffsetDateTime,
    pub extras: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub id: i64,
    pub message: String,
    pub priority: u8,
    pub title: Option<String>,
}

#[cfg(feature = "manage-messages")]
#[cfg_attr(docsrs, doc(cfg(feature = "manage-messages")))]
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct PagedMessages {
    pub messages: Vec<Message>,
    pub paging: Paging,
}

#[cfg(feature = "manage-messages")]
#[cfg_attr(docsrs, doc(cfg(feature = "manage-messages")))]
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Paging {
    pub limit: usize,
    pub next: Option<String>,
    pub since: i64,
    pub size: usize,
}

#[cfg(feature = "manage-plugins")]
#[cfg_attr(docsrs, doc(cfg(feature = "manage-plugins")))]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PluginConf {
    pub author: Option<String>,
    pub capabilities: Vec<String>,
    pub enabled: bool,
    pub id: i64,
    pub license: Option<String>,
    pub module_path: String,
    pub name: String,
    pub token: String,
    pub website: Option<String>,
}

#[cfg(feature = "manage-users")]
#[cfg_attr(docsrs, doc(cfg(feature = "manage-users")))]
#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct User {
    pub admin: bool,
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct VersionInfo {
    pub build_date: String,
    pub commit: String,
    pub version: String,
}
