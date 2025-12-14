mod graphql;
mod playground;

use crate::playground::my_playground_source;
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = match app::app().await {
        Ok(res) => res,
        Err(err) => {
            panic!("Failed to initialize app: {:?}", err);
        }
    };

    app::init_log();

    let user_api_handler = graphql::service::HttpHandler::new(app.clone()).await;
    let admin_api_handler = graphql::admin::HttpHandler::new(app.clone()).await;
    let port = app.env.port.clone();

    let app_factory = move || {
        let mut app = App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allowed_methods(["GET", "POST"])
                    .max_age(3600)
                    .supports_credentials(),
            )
            .app_data(Data::new(user_api_handler.clone()))
            .app_data(Data::new(admin_api_handler.clone()))
            .service(
                web::resource("/api/graphql")
                    .guard(guard::Post())
                    .to(user_api_graphql_route),
            )
            .service(
                web::resource("/api/admin/graphql")
                    .guard(guard::Post())
                    .to(admin_api_graphql_route),
            );

        let playground_paths = vec!["/api", "/api/admin"];
        for path in playground_paths {
            app = app.service(
                web::resource(format!("{}/playground", path))
                    .guard(guard::Get())
                    .to(move || async {
                        let path = path.to_string();
                        handle_playground(format!("{}/graphql", path).as_str())
                    }),
            );
        }

        app
    };

    if app.env.with_lambda {
        println!("listen as lambda function");
        lambda_web::run_actix_on_lambda(app_factory)
            .await
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
    } else {
        println!("listen as http server on port {}", port);
        HttpServer::new(app_factory)
            .bind(format!("127.0.0.1:{}", port))?
            .run()
            .await
    }
}

async fn user_api_graphql_route(
    handler: Data<graphql::service::HttpHandler>,
    http_req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    handler.handle(http_req, gql_req).await
}

async fn admin_api_graphql_route(
    handler: Data<graphql::admin::HttpHandler>,
    http_req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    handler.handle(http_req, gql_req).await
}

fn handle_playground(path: &str) -> actix_web::Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(my_playground_source(GraphQLPlaygroundConfig::new(&path))))
}
