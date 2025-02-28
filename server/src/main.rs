use axum::{
    routing::*,
    Router,
    middleware
};
use auth::*;
use server::*;
use handlers::{
    user::*,
    project::*,
    teams::*
};
use routes::*;
use tower_http::cors::{Any, CorsLayer};
use tower::ServiceBuilder;
use dotenvy::dotenv;
use std::env;
use tracing::{info, error};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {
    // enviroment variables
    dotenv().ok();
    fmt::init();
    let port: String = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(user_routes())
        .merge(project_routes())
        .merge(team_routes())
        .layer(ServiceBuilder::new().layer(cors_layer));
        // .layer(middleware::from_fn(auth_middleware));

    // Start server
    match tokio::net::TcpListener::bind(&addr).await {
        Ok(listener) => {
            info!("🚀 Server running on http://{}", addr);
            if let Err(e) = axum::serve(listener, app).await {
                error!("server error: {}", e);
            }
        }
        Err(e) => error!("failed to bind server addres {}: {}", addr, e)
    }
}
