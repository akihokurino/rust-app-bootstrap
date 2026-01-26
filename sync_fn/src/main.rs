use anyhow::anyhow;
use app::domain::types::task::{SyncTaskPayload, SyncTaskResponse};
use app::AppResult;
use lambda_runtime::{service_fn, Error, LambdaEvent};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::tracing::init_default_subscriber();
    lambda_runtime::run(service_fn(bridge)).await?;
    Ok(())
}

async fn bridge(event: LambdaEvent<SyncTaskPayload>) -> Result<SyncTaskResponse, Error> {
    let app = match app::app().await {
        Ok(res) => res,
        Err(err) => {
            panic!("Failed to initialize app: {:?}", err);
        }
    };

    let _sentry = app.error_notifier.init();
    app::init_log();

    let (request, _context) = event.into_parts();
    let result = exec(app, request).await;

    match result {
        Ok(response) => Ok(response),
        Err(err) => {
            tracing::error!("{:?}", err);
            app.error_notifier.send(err.clone());
            Err(anyhow!(err).into())
        }
    }
}

async fn exec(_app: &app::App, payload: SyncTaskPayload) -> AppResult<SyncTaskResponse> {

    tracing::info!("Task name: {}", payload.name);

    Ok(SyncTaskResponse { name: payload.name })
}
