use std::net::SocketAddr;

use axum::{Router, extract::Query, routing::get};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use utoipauto::utoipauto;

#[utoipauto]
#[derive(OpenApi)]
#[openapi(info(description = "My Api description"))]
pub struct ApiDoc;

#[derive(IntoParams, Deserialize)]
pub struct MyQueryParams {
    pub name: String,
}

#[utoipa::path(get, path = "/hello-world")]
async fn hello_world() -> &'static str {
    "Hello world"
}

#[utoipa::path(get, path = "/hello-with-name")]
async fn hello_with_name(Query(MyQueryParams { name }): Query<MyQueryParams>) -> String {
    format!("Hello, {}!", name)
}

pub fn gen_api_doc() -> String {
    ApiDoc::openapi()
        .to_pretty_json()
        .expect("Failed generated OpenAPI doc")
}

pub fn generate_openapi_spec() {
    const SPEC_PATH: &str = "openapi-spec.json";
    let spec = gen_api_doc();
    let path = std::path::Path::new(SPEC_PATH);
    std::fs::write(path, spec).expect("Failed to write OpenAPI spec");
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello-world", get(hello_world))
        .route("/hello-with-name", get(hello_with_name))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    generate_openapi_spec();
    let addr = SocketAddr::from(([127, 0, 0, 1], 1337));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
