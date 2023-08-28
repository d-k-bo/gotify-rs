use futures_util::StreamExt;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client: gotify::ClientClient = gotify::Client::new(
        &*std::env::var("GOTIFY_URL")?,
        std::env::var("GOTIFY_CLIENT_TOKEN")?,
    )?;
    let mut messages = client.stream_messages().await?;
    while let Some(result) = messages.next().await {
        let message = result?;
        println!("{message:#?}")
    }
    Ok(())
}
