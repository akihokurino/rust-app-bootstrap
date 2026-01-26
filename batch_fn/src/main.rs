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
    let app = match app::app().await {
        Ok(res) => res,
        Err(err) => {
            panic!("Failed to initialize app: {:?}", err);
        }
    };

    let _sentry = app.error_notifier.init();
    app::init_log();

    if let Err(err) = exec(app, event.payload).await {
        tracing::error!("{:?}", err);
        app.error_notifier.send(err.clone());
        return Err(anyhow!(err).into());
    }
    Ok(())
}

async fn exec(_app: &app::App, payload: Value) -> AppResult<()> {

    tracing::info!("Batch task started with payload: {:?}", payload);

    Ok(())
}
