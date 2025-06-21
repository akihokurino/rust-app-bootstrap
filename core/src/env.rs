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
        }
    }

    pub fn is_prod(&self) -> bool {
        self.env == "prod"
    }
}
