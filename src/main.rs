
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = planify_rs::App::new(tracing::Level::DEBUG)?;
    app.run().await?;
    Ok(())
}