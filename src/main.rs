use anyhow::Result;
use async_canceling::business_logic::long_running_async_task;

#[tokio::main]
async fn main() -> Result<()> {
    long_running_async_task().await;

    Ok(())
}
