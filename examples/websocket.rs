use futures_util::StreamExt;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let client: gotify::ClientClient =
        gotify::Client::new(env!("GOTIFY_URL"), env!("GOTIFY_CLIENT_TOKEN"))?;
    let mut messages = client.message_stream().await?;
    while let Some(result) = messages.next().await {
        let message = result?;
        println!("{message:#?}")
    }
    Ok(())
}
