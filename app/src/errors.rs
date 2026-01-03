use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;

use derive_more::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Display)]
pub enum Kind {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Duplicate,
    Internal,
}

impl Kind {
    pub fn default(self) -> AppError {
        let msg = match self {
            Kind::BadRequest => "不正なリクエストです",
            Kind::Unauthorized => "認証に失敗しました",
            Kind::Forbidden => "アクセスが許可されていません",
            Kind::NotFound => "リソースが見つかりません",
            Kind::Duplicate => "重複するリソースが存在します",
            Kind::Internal => "内部エラーが発生しました",
        };

        AppError {
            kind: self,
            msg: Some(msg.to_string()),
            src: None,
        }
    }

    pub fn with(self, msg: impl Into<String>) -> AppError {
        AppError {
            kind: self,
            msg: Some(msg.into()),
            src: None,
        }
    }

    #[inline]
    pub fn withf<T>(self) -> impl FnOnce(T) -> AppError
    where
        T: Into<String>,
    {
        move |v| self.with(v)
    }
    pub fn from_src(self, src: impl std::error::Error + Send + Sync + 'static) -> AppError {
        AppError {
            kind: self,
            msg: None,
            src: Some(Arc::from(src)),
        }
    }

    #[inline]
    pub fn from_srcf<T>(self) -> impl FnOnce(T) -> AppError
    where
        T: std::error::Error + Send + Sync + 'static,
    {
        move |v| self.from_src(v)
    }
    pub fn into_err(self) -> AppError {
        self.into()
    }
}

#[derive(Debug, Clone)]
pub struct AppError {
    pub kind: Kind,
    pub msg: Option<String>,
    pub src: Option<Arc<dyn std::error::Error + Send + Sync>>,
}
impl AppError {
    pub fn with_src(self, src: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self {
            src: Some(Arc::from(src)),
            ..self
        }
    }

    pub fn with_box_src(self, src: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Self {
            src: Some(Arc::from(src)),
            ..self
        }
    }
}
impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.kind,
            self.msg
                .as_ref()
                .map(|v| format!(": {}", v))
                .unwrap_or_default(),
            self.src
                .as_ref()
                .map(|v| format!(": {}", v))
                .unwrap_or_default()
        )
    }
}
impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.src
            .as_ref()
            .map(|v| v.as_ref() as &(dyn std::error::Error + 'static))
    }
}
impl From<Kind> for AppError {
    fn from(kind: Kind) -> Self {
        Self {
            kind,
            msg: None,
            src: None,
        }
    }
}
impl From<String> for AppError {
    fn from(value: String) -> Self {
        Self {
            kind: Kind::Internal,
            msg: Some(value),
            src: None,
        }
    }
}

pub trait NotFoundToNone<T> {
    fn not_found_to_none(self) -> Result<Option<T>, AppError>;
}
impl<T> NotFoundToNone<T> for Result<T, AppError> {
    fn not_found_to_none(self) -> Result<Option<T>, AppError> {
        match self {
            Ok(v) => Ok(Some(v)),
            Err(v) => match v.kind {
                Kind::NotFound => Ok(None),
                _ => Err(v),
            },
        }
    }
}

macro_rules! impl_from_err_to_app_internal_err {
    ($T:ty) => {
        impl From<$T> for crate::errors::AppError {
            fn from(v: $T) -> Self {
                crate::errors::Kind::Internal.from_src(v)
            }
        }
    };
}
pub(crate) use impl_from_err_to_app_internal_err;
