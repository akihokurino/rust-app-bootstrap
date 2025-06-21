mod errors;
mod types;

use crate::domain::S3Key;
use crate::errors::Kind::*;
use crate::infra::s3::types::{
    Condition, HeadObjectResponse, Policy, Session, Statement, StringLike,
};
use crate::AppResult;
use aws_sdk_s3::error::*;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use bytes::Bytes;
use http::Uri;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Adapter {
    client: Client,
    sts_cli: aws_sdk_sts::Client,
    default_bucket: String,
    asset_manager_role_arn: String,
}

impl Adapter {
    pub fn new(
        client: Client,
        sts_cli: aws_sdk_sts::Client,
        bucket: String,
        asset_manager_role_arn: String,
    ) -> Self {
        Self {
            client,
            sts_cli,
            default_bucket: bucket,
            asset_manager_role_arn,
        }
    }

    pub async fn pre_sign_for_upload(&self, key: &S3Key) -> AppResult<Uri> {
        let expires_in = Duration::from_secs(60 * 60);
        let pre_signed = self
            .client
            .put_object()
            .bucket(self.default_bucket.clone())
            .key(key.to_string().as_str())
            .presigned(PresigningConfig::expires_in(expires_in).unwrap())
            .await?;
        Ok(pre_signed
            .uri()
            .to_owned()
            .parse()
            .map_err(Internal.from_srcf())?)
    }

    pub async fn pre_sign_for_get(&self, key: &S3Key) -> AppResult<Uri> {
        let expires_in = Duration::from_secs(60 * 60);
        let pre_signed = self
            .client
            .get_object()
            .bucket(self.default_bucket.clone())
            .key(key.to_string().as_str())
            .presigned(PresigningConfig::expires_in(expires_in).unwrap())
            .await?;
        Ok(pre_signed
            .uri()
            .to_owned()
            .parse()
            .map_err(Internal.from_srcf())?)
    }

    pub async fn download_object(&self, key: &S3Key) -> AppResult<Bytes> {
        let resp = self
            .client
            .get_object()
            .bucket(self.default_bucket.clone())
            .key(key.to_string().as_str())
            .send()
            .await?;
        let data = resp.body.collect().await;
        Ok(data.unwrap().into_bytes())
    }

    pub async fn head_object(&self, key: &S3Key) -> AppResult<HeadObjectResponse> {
        let res = self
            .client
            .head_object()
            .bucket(self.default_bucket.clone())
            .key(key.to_string().as_str())
            .send()
            .await;

        match res {
            Err(SdkError::ServiceError(err)) if err.err().is_not_found() => {
                return Err(NotFound.into_err());
            }
            Err(e) => return Err(Internal.from_src(e)),
            _ => {}
        }

        Ok(HeadObjectResponse {
            s3_path: format!("s3://{}/{}", self.default_bucket, key),
        })
    }

    pub async fn copy_object(&self, src_key: &S3Key, dest_key: &S3Key) -> AppResult<()> {
        let res = self
            .client
            .copy_object()
            .bucket(self.default_bucket.clone())
            .copy_source(format!("{}/{}", self.default_bucket, src_key))
            .key(dest_key.to_string().as_str())
            .send()
            .await;

        match res {
            Err(e) => return Err(Internal.from_src(e)),
            Ok(v) if v.copy_object_result.is_none() => return Err(BadRequest.into_err()),
            _ => {}
        }
        Ok(())
    }

    pub async fn read_write_session(&self, path: &str, session_name: &str) -> AppResult<Session> {
        let policy = Policy::new()
            .add_statement(Statement::new(
                "Allow".to_string(),
                vec!["s3:GetObject".to_string()],
                format!("arn:aws:s3:::{}/{}/*", self.default_bucket, path),
                Default::default(),
            ))
            .add_statement(Statement::new(
                "Allow".to_string(),
                vec!["s3:ListBucket".to_string()],
                format!("arn:aws:s3:::{}", self.default_bucket),
                Some(Condition::new().string_like(StringLike::new().prefix(path.to_string()))),
            ))
            .add_statement(Statement::new(
                "Allow".to_string(),
                vec!["s3:PutObject".to_string(), "s3:PutObjectAcl".to_string()],
                format!("arn:aws:s3:::{}/{}/*", self.default_bucket, path),
                Default::default(),
            ));

        let res = self
            .sts_cli
            .assume_role()
            .role_session_name(session_name)
            .role_arn(self.asset_manager_role_arn.clone())
            .policy(serde_json::to_string(&policy).unwrap())
            .duration_seconds(3600)
            .send()
            .await
            .map_err(|err| Internal.with(err.to_string()))?;

        let credentials = res
            .credentials
            .ok_or(Internal.with("no credentials".to_string()))?;

        Ok(Session {
            bucket: self.default_bucket.clone(),
            prefix: path.to_string(),
            access_key_id: credentials.access_key_id,
            secret_access_key: credentials.secret_access_key,
            session_token: credentials.session_token,
        })
    }
}
