use std::sync::{Arc, OnceLock};

pub use macro_rules_attribute::apply;

use super::*;

pub const GOTIFY_URL: &str = "http://localhost:30080";
pub const GOTIFY_APP_TOKEN: &str = "AGo8b9paHo5wPkI";
pub const GOTIFY_CLIENT_TOKEN: &str = "C4er8DTiNk08mtt";

pub fn unauthenticated_client() -> Arc<UnauthenticatedClient> {
    static CLIENT: OnceLock<Arc<UnauthenticatedClient>> = OnceLock::new();

    CLIENT
        .get_or_init(|| Arc::new(UnauthenticatedClient::new_unauthenticated(GOTIFY_URL).unwrap()))
        .clone()
}

pub fn app_client() -> Arc<AppClient> {
    static CLIENT: OnceLock<Arc<AppClient>> = OnceLock::new();

    CLIENT
        .get_or_init(|| Arc::new(AppClient::new(GOTIFY_URL, GOTIFY_APP_TOKEN).unwrap()))
        .clone()
}

pub fn client_client() -> Arc<ClientClient> {
    static CLIENT: OnceLock<Arc<ClientClient>> = OnceLock::new();

    CLIENT
        .get_or_init(|| Arc::new(ClientClient::new(GOTIFY_URL, GOTIFY_CLIENT_TOKEN).unwrap()))
        .clone()
}

macro_rules! run_test_server {
    (
        #[test]
        async fn $fn_name:ident() -> $return_type:ty $body:block
    ) => {
        #[test]
        fn $fn_name() -> $return_type {
            crate::testsuite::start_test_server_with(async { $body })
        }
    };
}

pub(crate) use run_test_server;

pub fn start_test_server_with(
    fut: impl std::future::Future<Output = eyre::Result<()>> + Send + 'static,
) -> eyre::Result<()> {
    static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

    RUNTIME
        .get_or_init(|| tokio::runtime::Runtime::new().unwrap())
        .block_on(async {
            use futures_util::FutureExt;

            let mut server = start_server().await?;

            let result = std::panic::AssertUnwindSafe(fut).catch_unwind().await;

            server.kill()?;
            server.wait()?;

            match result {
                Ok(res) => res,
                Err(e) => std::panic::resume_unwind(e),
            }
        })
}

async fn start_server() -> eyre::Result<std::process::Child> {
    let test_server_dir =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/test-server");

    #[cfg(target_arch = "x86")]
    let arch = "386";
    #[cfg(target_arch = "x86_64")]
    let arch = "amd64";
    #[cfg(target_arch = "arm")]
    let arch = "arm-7";
    #[cfg(target_arch = "aarch64")]
    let arch = "arm64";
    #[cfg(not(any(
        target_arch = "x86",
        target_arch = "x86_64",
        target_arch = "arm",
        target_arch = "aarch64"
    )))]
    compile_error!("Your architecture seems to be unsupported by gotify-server. If this assumption is incorrect, please create an issue on github.");

    #[cfg(target_os = "linux")]
    let gotify_binary = format!("gotify-linux-{arch}");
    #[cfg(target_os = "windows")]
    let gotify_binary = format!("gotify-windows-{arch}.exe");
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    compile_error!("Your operating system seems to be unsupported by gotify-server. If this assumption is incorrect, please create an issue on github.");

    let gotify_binary_path = test_server_dir.join(&gotify_binary);

    if !gotify_binary_path.try_exists()? {
        let client = reqwest::Client::new();
        let download_url = client
            .get("https://api.github.com/repos/gotify/server/releases/latest")
            .header("User-Agent", "github.com/d-k-bo/gotify-rs testsuite")
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?
            .get("assets")
            .and_then(serde_json::Value::as_array)
            .and_then(|assets| {
                assets.iter().find(|asset| {
                    asset
                        .get("name")
                        .and_then(serde_json::Value::as_str)
                        .and_then(|name| name.strip_suffix(".zip"))
                        .is_some_and(|name| name == gotify_binary)
                })
            })
            .and_then(|asset| {
                asset
                    .get("browser_download_url")
                    .and_then(serde_json::Value::as_str)
            })
            .ok_or_else(|| eyre::eyre!("failed to find latest gotify binary"))?
            .to_owned();

        zip::ZipArchive::new(std::io::Cursor::new(
            client.get(download_url).send().await?.bytes().await?,
        ))?
        .extract(&test_server_dir)?;
    }

    let data_dir = test_server_dir.join("data");
    std::fs::create_dir_all(&data_dir)?;

    std::fs::copy(
        test_server_dir.join("gotify.db"),
        data_dir.join("gotify.db"),
    )?;

    let server = std::process::Command::new(gotify_binary_path.canonicalize()?)
        .env("GOTIFY_SERVER_PORT", "30080")
        .stdout(std::fs::File::create(test_server_dir.join("gotify.log"))?)
        .current_dir(test_server_dir)
        .spawn()?;

    loop {
        match reqwest::get("http://localhost:30080").await {
            Ok(_) => return Ok(server),
            Err(e) if e.is_connect() => {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await
            }
            Err(e) => return Err(e.into()),
        }
    }
}

#[test]
#[should_panic]
fn self_test() {
    start_test_server_with(async {
        assert_eq!(true, false);

        Ok(())
    })
    .unwrap()
}
