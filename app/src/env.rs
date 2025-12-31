use dotenv::dotenv;
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

    // Google Cloud関連を使う場合は必須
    pub google_project_id: Option<String>,
    pub google_application_credentials: Option<oauth2::ServiceAccountKey>,

    // ImageOptimizerを使う場合は必須
    pub cloudfront_domain: Option<String>,
    pub cloudfront_key_pair_id: Option<String>,
    pub cloudfront_private_key: Option<String>,
}
impl Env {
    pub fn new() -> Self {
        if Self::is_local() {
            dotenv().ok();
        }

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
            database_url: must_env("DATABASE_URL"),
            s3_bucket_name: must_env("S3_BUCKET_NAME"),
            sns_async_task_topic_arn: must_env("SNS_ASYNC_TASK_TOPIC_ARN"),
            sqs_async_task_queue_url: must_env("SQS_ASYNC_TASK_QUEUE_URL"),
            sync_task_lambda_arn: std::env::var("SYNC_TASK_LAMBDA_ARN").unwrap_or("".to_string()), // TODO: input target lambda arn
            cognito_admin_user_pool_id: must_env("COGNITO_ADMIN_USER_POOL_ID"),

            // Google Cloud関連を使う場合は必須
            google_project_id: std::env::var("GOOGLE_PROJECT_ID").ok(),
            google_application_credentials: google_service_account,

            // ImageOptimizerを使う場合は必須
            cloudfront_domain: std::env::var("CLOUDFRONT_DOMAIN").ok(),
            cloudfront_key_pair_id: std::env::var("CLOUDFRONT_KEY_PAIR_ID").ok(),
            cloudfront_private_key: std::env::var("CLOUDFRONT_PRIVATE_KEY").ok(),
        }
    }

    pub fn is_prod(&self) -> bool {
        self.env == "prod"
    }

    pub fn is_local() -> bool {
        std::env::var("IS_LOCAL")
            .map(|v| bool::from_str(&v).expect("failed to parse IS_LOCAL"))
            .unwrap_or(false)
    }
}
