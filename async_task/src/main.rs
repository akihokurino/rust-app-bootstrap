use anyhow::anyhow;
use core::errors::Kind::BadRequest;
use core::infra::sns::types::{AsyncTaskPayload, EventData};
use core::AppResult;
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

#[tracing::instrument]
async fn exec(payload: Value) -> AppResult<()> {
    let _resolver = match core::resolver().await {
        Ok(res) => res,
        Err(err) => {
            panic!("Failed to initialize resolver: {:?}", err);
        }
    };

    let data: EventData = serde_json::from_value(payload)
        .map_err(|e| BadRequest.with("failed to parse payload").with_src(e))?;
    for record in data.records {
        let task_payload: AsyncTaskPayload = serde_json::from_str(&record.sns.message)
            .map_err(|e| BadRequest.with("failed to parse message").with_src(e))?;

        // task_payload.name でアクセス可能
        println!("Task name: {}", task_payload.name);
    }

    Ok(())
}
