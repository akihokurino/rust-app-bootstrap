mod query;

use crate::graphql::GraphResult;
use crate::graphql::admin::query::QueryRoot;
use crate::graphql::shared::schema::new_schema_builder;
use actix_web::HttpRequest;
use actix_web::http::header::HeaderValue;
use app::adapter::AdminAuth;
use app::errors::AppError;
use app::errors::Kind::BadRequest;
use app::errors::Kind::Unauthorized;
use app::{AppResult, domain};
use async_graphql::{Context, EmptyMutation, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use std::sync::Arc;

type AuthorizedUserId = domain::admin_user::Id;

#[async_trait]
trait AppContext {
    fn verified_user_id(&self) -> GraphResult<AuthorizedUserId>;
}
#[async_trait]
impl<'a> AppContext for Context<'_> {
    fn verified_user_id(&self) -> GraphResult<AuthorizedUserId> {
        match self.data::<AppResult<AuthorizedUserId>>()? {
            Ok(v) => Ok(v.clone()),
            Err(err) => Err(match err.kind {
                _ => Unauthorized
                    .with(format!("authorization error: {}", err))
                    .into(),
            }),
        }
    }
}

pub type Schema = async_graphql::Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[derive(Clone)]
pub struct HttpHandler {
    schema: Schema,
    auth: Option<Arc<dyn AdminAuth>>,
}

impl HttpHandler {
    pub async fn new(app: app::App) -> Self {
        let schema = new_schema_builder(app.clone(), QueryRoot::default(), EmptyMutation).finish();

        HttpHandler {
            schema,
            auth: Some(app.admin_auth),
        }
    }

    pub async fn handle(&self, http_req: HttpRequest, gql_req: GraphQLRequest) -> GraphQLResponse {
        let mut gql_req = gql_req.into_inner();

        let headers = http_req.headers();
        gql_req = gql_req.data(match (headers.get("authorization"), self.auth.as_ref()) {
            (Some(hv), Some(auth)) => verify_token(auth.as_ref(), hv).await,
            _ => Err(Unauthorized.into()),
        });

        if let Some(hv) = headers.get("x-debug-user-id") {
            if let Some(v) = hv.to_str().ok() {
                gql_req = gql_req.data(Ok::<AuthorizedUserId, AppError>(v.to_string().into()));
            }
        }

        self.schema.execute(gql_req).await.into()
    }
}

async fn verify_token(auth: &dyn AdminAuth, hv: &HeaderValue) -> AppResult<AuthorizedUserId> {
    let token_str = hv
        .to_str()
        .map_err(BadRequest.from_srcf())?
        .strip_prefix("Bearer ")
        .ok_or_else(|| BadRequest.with("invalid authorization header"))?;

    let user = auth.verify(token_str).await?;
    Ok(user.id.into())
}
