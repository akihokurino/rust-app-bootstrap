mod mutation;
mod query;
mod types;

use crate::graphql::service::mutation::MutationRoot;
use crate::graphql::service::query::QueryRoot;
use crate::graphql::GraphResult;
use crate::graphql::shared::schema::new_schema_builder;
use actix_web::http::header::HeaderValue;
use actix_web::HttpRequest;
use app::adapter::UserAuth;
use app::domain;
use app::errors::Kind::BadRequest;
use app::errors::Kind::Unauthorized;
use app::errors::{AppError, NotFoundToNone};
use app::AppResult;
use async_graphql::{Context, EmptySubscription};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::OnceCell;

type AuthorizedUserId = domain::user::Id;
type AuthorizedUser = OnceCell<domain::user::User>;

#[async_trait]
trait AppContext {
    fn verified_user_id(&self) -> GraphResult<AuthorizedUserId>;
    async fn verified_user(&self) -> GraphResult<domain::user::User>;
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

    async fn verified_user(&self) -> GraphResult<domain::user::User> {
        let cell = self.data::<AuthorizedUser>()?;

        cell.get_or_try_init(|| async {
            let verified_user_id = self.verified_user_id()?;
            let app = self.data::<app::App>()?;

            let me = app
                .user_repository
                .get(app.db_session.conn(), &verified_user_id)
                .await
                .not_found_to_none()?;
            match me {
                None => Err(Unauthorized.with("ユーザーが存在しません").into()),
                Some(v) => Ok(v),
            }
        })
        .await
        .cloned()
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
        let schema =
            new_schema_builder(app.clone(), QueryRoot::default(), MutationRoot::default()).finish();

        HttpHandler {
            schema,
            auth: app.user_auth,
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

        gql_req = gql_req.data(AuthorizedUser::new());

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
