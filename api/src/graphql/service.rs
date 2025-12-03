mod mutation;
mod query;
mod types;

use crate::graphql::service::mutation::MutationRoot;
use crate::graphql::service::query::QueryRoot;
use crate::graphql::{data_loader, GraphResult};
use actix_web::http::header::{HeaderMap, HeaderValue};
use actix_web::HttpRequest;
use app::domain;
use app::errors::AppError;
use app::errors::Kind::BadRequest;
use app::errors::Kind::Unauthorized;
use app::AppResult;
use async_graphql::{Context, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;

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
}

impl HttpHandler {
    pub async fn new() -> Self {
        let app = app::app().await.expect("Failed to initialize app").clone();
        let schema = Schema::build(
            QueryRoot::default(),
            MutationRoot::default(),
            EmptySubscription,
        )
        .data(app.clone())
        .data(data_loader::user::new_loader(app.clone()))
        .data(data_loader::order::new_loader(app.clone()))
        .finish();

        HttpHandler { schema }
    }

    pub async fn handle(&self, http_req: HttpRequest, gql_req: GraphQLRequest) -> GraphQLResponse {
        let mut gql_req = gql_req.into_inner();

        let headers: HeaderMap = HeaderMap::from_iter(http_req.headers().clone().into_iter());
        gql_req = gql_req.data(match headers.get("authorization") {
            None => Err(Unauthorized.into()),
            Some(hv) => verify_token(hv).await,
        });

        if let Some(hv) = headers.get("x-debug-user-id") {
            if let Some(v) = hv.to_str().ok() {
                gql_req = gql_req.data(Ok::<AuthorizedUserId, AppError>(v.to_string().into()));
            }
        }

        self.schema.execute(gql_req).await.into()
    }
}

async fn verify_token(hv: &HeaderValue) -> AppResult<AuthorizedUserId> {
    let _token_str = hv
        .to_str()
        .map_err(BadRequest.from_srcf())?
        .strip_prefix("Bearer ")
        .ok_or_else(|| BadRequest.with("invalid authorization header"))?;

    // TODO : Implement token verification logic
    let auth_id: Option<AuthorizedUserId> = None;
    auth_id.map_or(Err(BadRequest.with("invalid token")), |v| Ok(v))
}
