use axum::{
    extract::Path,
    response::Html,
    body::Body,
    routing::get,
    response::Json,
    Router,
};
use serde_json::{Value, json};
use self::models::*;
use diesel::prelude::*;
use server::*;
use handlers::*;
use tower_http::cors::{Any, CorsLayer};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {

    let cors_layer = CorsLayer::new()
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get("GAAA"))
        .route("/posts", get(show_posts))
        .route("/posts/{id}", get(get_post))
        .layer(ServiceBuilder::new().layer(cors_layer));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
