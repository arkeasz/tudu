use axum::{
    extract::Path,
    response::Html,
    body::Body,
    routing::*,
    response::Json,
    Router,
};
use serde_json::{Value, json};
use self::models::*;
use diesel::prelude::*;
use server::*;
use handlers::{
    user::*
};
use tower_http::cors::{Any, CorsLayer};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {

    let cors_layer = CorsLayer::new()
        .allow_origin(Any);

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/users", get(show_users))
        .route("/users/create", post(create_user))
        .route("/users/update/{id}", patch(update_user))
        .route("/users/delete/{id}", delete(delete_user))
        .layer(ServiceBuilder::new().layer(cors_layer));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
