use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::models::{Post, NewPost};
use crate::schema::posts::dsl::*;

type DbPool = Arc<Mutex<MysqlConnection>>;

#[derive(Serialize, Deserialize)]
struct PostPayload {
    title: String,
    body: String,
}

pub async fn get_posts(State(pool): State<DbPool>) -> impl IntoResponse {
    let connection = pool.lock().unwrap();
    match posts.load::<Post>(&*connection) {
        Ok(results) => (StatusCode::OK, Json(results)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn get_post_by_id(State(pool): State<DbPool>, axum::extract::Path(post_id): axum::extract::Path<i32>) -> impl IntoResponse {
    let connection = pool.lock().unwrap();
    match posts.find(post_id).first::<Post>(&*connection).optional() {
        Ok(Some(post)) => (StatusCode::OK, Json(post)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn create_post(State(pool): State<DbPool>, Json(payload): Json<PostPayload>) -> impl IntoResponse {
    let connection = pool.lock().unwrap();
    let new_post = NewPost {
        title: payload.title,
        body: payload.body,
        published: false,
    };

    match diesel::insert_into(posts)
        .values(&new_post)
        .execute(&*connection)
    {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
