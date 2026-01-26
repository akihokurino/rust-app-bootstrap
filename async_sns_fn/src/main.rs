use anyhow::anyhow;
use app::domain::types::task::{AsyncTaskPayload, SnsEventData};
use app::errors::Kind::BadRequest;
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

    let data: SnsEventData = serde_json::from_value(payload)
        .map_err(|e| BadRequest.with("failed to parse payload").with_src(e))?;
    if let Some(record) = data.records.first() {
        let task_payload: AsyncTaskPayload = serde_json::from_str(&record.sns.message)
            .map_err(|e| BadRequest.with("failed to parse message").with_src(e))?;

        tracing::info!("Task name: {}", task_payload.name);
    }

    Ok(())
}
