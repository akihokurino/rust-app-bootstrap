use anyhow::anyhow;
use app::domain::types::task::{AsyncTaskPayload, EventData};
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
    if let Err(err) = exec(event.payload).await {
        return Err(anyhow!(err).into());
    }
    Ok(())
}

async fn exec(payload: Value) -> AppResult<()> {
    let _app = match app::app().await {
        Ok(res) => res,
        Err(err) => {
            panic!("Failed to initialize app: {:?}", err);
        }
    };

    let data: EventData = serde_json::from_value(payload)
        .map_err(|e| BadRequest.with("failed to parse payload").with_src(e))?;
    if let Some(record) = data.records.first() {
        let task_payload: AsyncTaskPayload = serde_json::from_str(&record.sns.message)
            .map_err(|e| BadRequest.with("failed to parse message").with_src(e))?;

        println!("Task name: {}", task_payload.name);
    }

    Ok(())
}
