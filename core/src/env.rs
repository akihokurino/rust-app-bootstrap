use std::str::FromStr;

fn must_env(k: &str) -> String {
    std::env::var(k).expect(format!("env {} missing", k).as_str())
}

#[derive(Debug, Clone)]
pub struct Environments {
    pub env: String,
    pub port: String,
    pub with_lambda: bool,
    pub database_url: String,
    pub s3_bucket_name: String,
    pub sns_async_task_topic_arn: String,
}
impl Environments {
    pub fn new() -> Self {
        Environments {
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
        }
    }

    pub fn is_prod(&self) -> bool {
        self.env == "prod"
    }
}
