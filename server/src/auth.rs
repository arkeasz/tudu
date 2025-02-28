use axum::{routing::*, Router, middleware};
use tower_http::cors::{Any, CorsLayer};
use tower::ServiceBuilder;
use std::env;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use axum::extract::{Json, Extension};
use axum::http::StatusCode;
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use rand::Rng;
use axum::body::Body;
use axum::middleware::Next;
use axum::response::Response;
use axum::http::Request;

pub async fn auth_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let token = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .map(|h| h.trim_start_matches("Bearer "));

    if let Some(token) = token {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        match decode::<Claims>(token, &decoding_key, &Validation::default()) {
            Ok(_) => Ok(next.run(req).await),
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}
