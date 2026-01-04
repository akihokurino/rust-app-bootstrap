mod mutation;
mod query;
mod types;

use crate::graphql::service::mutation::MutationRoot;
use crate::graphql::service::query::QueryRoot;
use crate::graphql::{data_loader, GraphResult};
use actix_web::http::header::{HeaderMap, HeaderValue};
use actix_web::HttpRequest;
use app::adapter::UserAuth;
use app::domain;
use app::errors::AppError;
use app::errors::Kind::BadRequest;
use app::errors::Kind::Unauthorized;
use app::AppResult;
use async_graphql::{Context, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use std::sync::Arc;

type AuthorizedUserId = domain::user::Id;

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

pub type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Clone)]
pub struct HttpHandler {
    schema: Schema,
    auth: Option<Arc<dyn UserAuth>>,
}

impl HttpHandler {
    pub async fn new(app: app::App) -> Self {
        let schema = Schema::build(
            QueryRoot::default(),
            MutationRoot::default(),
            EmptySubscription,
        )
        .data(app.clone())
        .data(data_loader::new_user_loader(app.clone()))
        .data(data_loader::new_order_loader(app.clone()))
        .finish();

        HttpHandler {
            schema,
            auth: app.user_auth,
        }
    }

    pub async fn handle(&self, http_req: HttpRequest, gql_req: GraphQLRequest) -> GraphQLResponse {
        let mut gql_req = gql_req.into_inner();

        let headers: HeaderMap = HeaderMap::from_iter(http_req.headers().clone().into_iter());
        gql_req = gql_req.data(match (headers.get("authorization"), self.auth.clone()) {
            (Some(hv), Some(auth)) => verify_token(&*auth, hv).await,
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

async fn verify_token(auth: &dyn UserAuth, hv: &HeaderValue) -> AppResult<AuthorizedUserId> {
    let token_str = hv
        .to_str()
        .map_err(BadRequest.from_srcf())?
        .strip_prefix("Bearer ")
        .ok_or_else(|| BadRequest.with("invalid authorization header"))?;

    let uid = auth.verify(token_str).await?;
    Ok(uid)
}
