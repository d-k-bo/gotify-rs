# gotify-rs

[![Build Status](https://github.com/d-k-bo/gotify-rs/workflows/CI/badge.svg)](https://github.com/d-k-bo/gotify-rs/actions?query=workflow%3ACI)
[![Crates.io](https://img.shields.io/crates/v/gotify)](https://lib.rs/crates/gotify)
[![Documentation](https://img.shields.io/docsrs/gotify)](https://docs.rs/gotify)
[![License: MIT](https://img.shields.io/crates/l/gotify)](LICENSE)

<!-- cargo-rdme start -->

An idiomatic Rust client for Gotify.

### Overview

By default, this crate only exposes the `Client::health()`,
`Client::version()` methods.
All other categories of endpoints must be enabled by the corresponding feature flags.

<details><summary>Table of available feature flags</summary>

| Feature flag | Enabled methods | Note |
| ------------ | --------------- | ---- |
| `app` | `Client::create_message()` | |
| `manage-applications` | `Client::get_applications()`, `Client::create_application()`, `Client::update_application()`, `Client::delete_application()`, `Client::delete_application_image()` | |
| `manage-clients` | `Client::get_clients()`, `Client::create_client()`, `Client::update_client()`, `Client::delete_client()` | |
| `manage-messages` | `Client::get_application_messages()`, `Client::delete_application_messages()`, `Client::get_messages()`, `Client::delete_messages()`, `Client::delete_message()` | doesn't include `Client::create_message()` and `Client::stream_messages()` |
| `manage-plugins` | `Client::get_plugins()`, `Client::get_plugin_config()`, `Client::update_plugin_config()`, `Client::disable_plugin()`, `Client::get_plugin_display()`, `Client::enable_plugin()` | |
| `manage-users` | `Client::get_current_user()`, `Client::update_current_user()`, `Client::get_users()`, `Client::get_user()`, `Client::update_user()`, `Client::delete_user()` | |
| `websocket` | `Client::stream_messages()` | enables additional dependencies (mainly [`tokio-tungstenite`](https://docs.rs/tokio-tungstenite)) |

</details>

Most methods that send data to Gotify's API use the
[builder pattern](https://rust-unofficial.github.io/patterns/patterns/creational/builder.html)
for a more readable API and better support of future additions to Gotify's API.
If an optional parameter is added to an endpoint, it can be be added
as a builder method without causing to much breakage.
All builders implement [`IntoFuture`](std::future::IntoFuture), so those
methods can also be `await`ed directly, just as if they were regular async methods.

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

let mut messages = client.stream_messages().await?;

while let Some(result) = messages.next().await {
    let message = result?;

    println!("{message:#?}")
}
```

<!-- cargo-rdme end -->

## License

This project is licensed under the MIT License.

See [LICENSE](LICENSE) for more information.
