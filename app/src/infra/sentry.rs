use crate::adapter::ErrorNotifier;
use crate::errors::{AppError, Kind};
use sentry::protocol::{Event, Exception};
use std::sync::Arc;

pub struct InitGuard(Arc<sentry::Client>);
impl Drop for InitGuard {
    fn drop(&mut self) {
        sentry::end_session();
        self.0.close(None);
    }
}

#[derive(Clone, Debug)]
pub struct Adapter {
    pub client: sentry::Client,
    pub is_local: bool,
}

impl Adapter {
    pub fn new(client: sentry::Client, is_local: bool) -> Self {
        Self { client, is_local }
    }
}

impl ErrorNotifier for Adapter {
    fn init(&self) -> InitGuard {
        let client = Arc::new(self.client.clone());
        sentry::Hub::current().bind_client(Some(client.clone()));
        if client.options().auto_session_tracking
            && client.options().session_mode == sentry::SessionMode::Application
        {
            sentry::start_session()
        }
        InitGuard(client)
    }

    fn send(&self, err: AppError) {
        if self.is_local {
            return;
        }

        let level = match err.kind {
            Kind::Internal => sentry::Level::Error,
            Kind::BadRequest | Kind::Unauthorized | Kind::Forbidden | Kind::Duplicate => {
                sentry::Level::Warning
            }
            Kind::NotFound => sentry::Level::Info,
        };

        let location_str = format!(
            "{}:{}:{}",
            err.location.file(),
            err.location.line(),
            err.location.column()
        );

        let error_message = err
            .msg
            .clone()
            .or_else(|| err.src.as_ref().map(|s| s.to_string()))
            .unwrap_or_else(|| err.kind.to_string());

        let exception = Exception {
            ty: err.kind.to_string(),
            value: Some(error_message.clone()),
            ..Default::default()
        };

        let mut event = Event {
            message: Some(format!(
                "[{}] {}: {}",
                location_str, err.kind, error_message
            )),
            level,
            exception: vec![exception].into(),
            ..Default::default()
        };

        event.tags.insert("error.kind".into(), err.kind.to_string());
        event
            .tags
            .insert("error.file".into(), err.location.file().into());
        event
            .tags
            .insert("error.line".into(), err.location.line().to_string());

        event.extra.insert(
            "location".into(),
            sentry::protocol::Value::from(location_str),
        );
        event.extra.insert(
            "file".into(),
            sentry::protocol::Value::from(err.location.file()),
        );
        event.extra.insert(
            "line".into(),
            sentry::protocol::Value::from(err.location.line() as i64),
        );
        event.extra.insert(
            "column".into(),
            sentry::protocol::Value::from(err.location.column() as i64),
        );
        if let Some(ref msg) = err.msg {
            event.extra.insert(
                "error.message".into(),
                sentry::protocol::Value::from(msg.clone()),
            );
        }
        if let Some(ref src) = err.src {
            event.extra.insert(
                "error.source".into(),
                sentry::protocol::Value::from(src.to_string()),
            );
        }

        sentry::capture_event(event);
    }
}
