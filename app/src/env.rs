use google_identitytoolkit3::oauth2;
use std::str::FromStr;

fn must_env(k: &str) -> String {
    std::env::var(k).expect(format!("env {} missing", k).as_str())
}

#[derive(Debug, Clone)]
pub struct Env {
    pub env: String,
    pub port: String,
    pub with_lambda: bool,
    pub database_url: String,
    pub s3_bucket_name: String,
    pub sns_async_task_topic_arn: String,
    pub sqs_async_task_queue_url: String,
    pub sync_task_lambda_arn: String,
    pub cognito_admin_user_pool_id: String,
    pub google_project_id: String,
    pub google_application_credentials: Option<oauth2::ServiceAccountKey>,
    pub cloudfront_domain: Option<String>,
    pub cloudfront_key_pair_id: Option<String>,
    pub cloudfront_private_key: Option<String>,
}
impl Env {
    pub fn new() -> Self {
        let google_service_account =
            if let Some(cred) = std::env::var("GOOGLE_APPLICATION_CREDENTIALS_BODY").ok() {
                let parsed: Option<oauth2::ServiceAccountKey> =
                    serde_json::from_str(cred.as_str()).ok();
                parsed
            } else {
                None
            };

        Env {
            env: must_env("ENV"),
            port: std::env::var("PORT").unwrap_or("8080".to_string()),
            with_lambda: std::env::var("WITH_LAMBDA")
                .map(|v| bool::from_str(&v).expect("failed to parse WITH_LAMBDA"))
                .unwrap_or(false),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or("postgresql://postgres:postgres@localhost:5432/app".to_string()),
            s3_bucket_name: std::env::var("S3_BUCKET_NAME")
                .expect("failed to parse S3_BUCKET_NAME"),
            sns_async_task_topic_arn: std::env::var("SNS_ASYNC_TASK_TOPIC_ARN")
                .expect("failed to parse SNS_ASYNC_TASK_TOPIC_ARN"),
            sqs_async_task_queue_url: std::env::var("SQS_ASYNC_TASK_QUEUE_URL")
                .expect("failed to parse SQS_ASYNC_TASK_QUEUE_URL"),
            sync_task_lambda_arn: std::env::var("SYNC_TASK_LAMBDA_ARN").unwrap_or("".to_string()), // TODO: input target lambda arn
            cognito_admin_user_pool_id: std::env::var("COGNITO_ADMIN_USER_POOL_ID")
                .expect("failed to parse COGNITO_ADMIN_USER_POOL_ID"),
            google_project_id: must_env("GOOGLE_PROJECT_ID"),
            google_application_credentials: google_service_account,
            cloudfront_domain: std::env::var("CLOUDFRONT_DOMAIN").ok(),
            cloudfront_key_pair_id: std::env::var("CLOUDFRONT_KEY_PAIR_ID").ok(),
            cloudfront_private_key: std::env::var("CLOUDFRONT_PRIVATE_KEY").ok(),
        }
    }

    pub fn is_prod(&self) -> bool {
        self.env == "prod"
    }
}
