#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client: gotify::AppClient = gotify::Client::new(
        &*std::env::var("GOTIFY_URL")?,
        std::env::var("GOTIFY_APP_TOKEN")?,
    )?;

    client
        .create_message("Lorem ipsum dolor sit amet")
        .with_title("Lorem Ipsum")
        .await?;
    Ok(())
}
