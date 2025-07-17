use anyhow::anyhow;
use app::AppResult;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::tracing::init_default_subscriber();
    lambda_runtime::run(service_fn(bridge)).await?;
    Ok(())
}

async fn bridge(event: LambdaEvent<Value>) -> Result<(), Error> {
    if let Err(err) = exec(event.payload).await {
        return Err(anyhow!(err).into());
    }
    Ok(())
}

async fn exec(payload: Value) -> AppResult<()> {
    let _resolver = match app::resolver().await {
        Ok(res) => res,
        Err(err) => {
            panic!("Failed to initialize resolver: {:?}", err);
        }
    };

    println!("Batch task started with payload: {:?}", payload);

    Ok(())
}
