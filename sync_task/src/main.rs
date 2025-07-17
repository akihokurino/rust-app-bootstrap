use anyhow::anyhow;
use app::infra::lambda::types::SyncTaskPayload;
use app::infra::lambda::types::SyncTaskResponse;
use app::AppResult;
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::tracing::init_default_subscriber();
    lambda_runtime::run(service_fn(bridge)).await?;
    Ok(())
}

async fn bridge(event: LambdaEvent<SyncTaskPayload>) -> Result<SyncTaskResponse, Error> {
    let (request, _context) = event.into_parts();
    let result = exec(request).await;

    match result {
        Ok(response) => Ok(response),
        Err(err) => Err(anyhow!(err).into()),
    }
}

async fn exec(payload: SyncTaskPayload) -> AppResult<SyncTaskResponse> {
    let _resolver = match app::resolver().await {
        Ok(res) => res,
        Err(err) => {
            panic!("Failed to initialize resolver: {:?}", err);
        }
    };

    println!("Task name: {}", payload.name);

    Ok(SyncTaskResponse { name: payload.name })
}
