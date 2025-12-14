mod errors;
pub mod types;

use crate::adapter::Storage;
use crate::domain::types::s3_key::S3Key;
use crate::errors::Kind::*;
use crate::infra::s3::types::HeadObjectResponse;
use crate::AppResult;
use async_trait::async_trait;
use aws_sdk_s3::error::*;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use bytes::Bytes;
use http::Uri;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Adapter {
    client: Client,
    default_bucket: String,
}

impl Adapter {
    pub fn new(client: Client, bucket: String) -> Self {
        Self {
            client,
            default_bucket: bucket,
        }
    }
}

#[async_trait]
impl Storage for Adapter {
    async fn presign_for_upload(&self, key: &S3Key) -> AppResult<Uri> {
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

    async fn presign_for_get(&self, key: &S3Key) -> AppResult<Uri> {
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

    async fn download_object(&self, key: &S3Key) -> AppResult<Bytes> {
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

    async fn head_object(&self, key: &S3Key) -> AppResult<HeadObjectResponse> {
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

    async fn copy_object(&self, src_key: &S3Key, dest_key: &S3Key) -> AppResult<()> {
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
}
