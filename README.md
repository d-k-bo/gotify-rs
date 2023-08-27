# gotify-rs

[![Build Status](https://github.com/d-k-bo/gotify/workflows/CI/badge.svg)](https://github.com/d-k-bo/gotify/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/gotify)](https://lib.rs/crates/gotify)
[![Documentation](https://img.shields.io/docsrs/gotify)](https://docs.rs/gotify)
[![License: MIT](https://img.shields.io/crates/l/gotify)](LICENSE)

<!-- cargo-rdme start -->

An idiomatic Rust client for Gotify.

### Overview

By default, this crate only exposes the `Client::health()`, `Client::version()` methods.
All other categories of endpoints must be enabled by the correspondig feature flags.

| Feature flag | Enabled methods | Note |
| ------------ | --------------- | ---- |
| `app` | `Client::create_message()` | |
| `manage-clients` | `Client::get_clients()`, `Client::create_client()`, `Client::update_client()`, `Client::delete_client()` | |
| `manage-messages` | `Client::get_application_messages()`, `Client::delete_application_messages()`, `Client::get_messages()`, `Client::delete_messages()`, `Client::delete_message()` | doesn't include `Client::create_message()` and `Client::message_stream()` |
| `manage-plugins` | `Client::get_plugins()`, `Client::get_plugin_config()`, `Client::update_plugin_config()`, `Client::disable_plugin()`, `Client::get_plugin_display()`, `Client::enable_plugin()` | |
| `websocket` | `Client::message_stream()` | enables additional dependencies (mainly [`tokio-tungstenite`](https://docs.rs/tokio-tungstenite)) |

### Examples

#### Creating a message

```rust
let client: gotify::AppClient = gotify::Client::new(GOTIFY_URL, GOTIFY_APP_TOKEN)?;

client.create_message("Lorem ipsum dolor sit amet").with_title("Lorem Ipsum").await?;
```

#### Listening for new messages

```rust
use futures_util::StreamExt;

let client: gotify::ClientClient = gotify::Client::new(GOTIFY_URL, GOTIFY_CLIENT_TOKEN)?;

let mut messages = client.message_stream().await?;

while let Some(result) = messages.next().await {
    let message = result?;

    println!("{message:#?}")
}
```

<!-- cargo-rdme end -->

## License

This project is licensed under the MIT License.

See [LICENSE](LICENSE) for more information.
