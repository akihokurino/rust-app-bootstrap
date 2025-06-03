mod graphql;

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let api_http_handler = graphql::service::HttpHandler::new().await;
    let port = 8080; // Default port

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
            .app_data(Data::new(api_http_handler.clone()))
            .service(
                web::resource("/api/graphql")
                    .guard(guard::Post())
                    .to(api_graphql_route),
            );

        app = app.service(
            web::resource("/api/playground")
                .guard(guard::Get())
                .to(|| async { handle_playground("api") }),
        );

        app
    };

    println!("listen as http server on port {}", port);
    HttpServer::new(app_factory)
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await
}

async fn api_graphql_route(
    handler: Data<graphql::service::HttpHandler>,
    http_req: HttpRequest,
    gql_req: GraphQLRequest,
) -> GraphQLResponse {
    handler.handle(http_req, gql_req).await
}

fn handle_playground(schema_name: &'static str) -> actix_web::Result<HttpResponse> {
    let path = format!("/{}/graphql", schema_name);
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new(&path))))
}
