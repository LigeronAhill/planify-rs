
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = planify_rs::App::new(tracing::Level::DEBUG).await?;
    if let Err(e) = app.run().await {
        tracing::error!("Ошибка в процессе выполнения программы: {e:?}");
        app.shutdown().await?;
    }
    Ok(())
}