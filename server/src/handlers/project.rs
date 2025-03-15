use crate::models::{
    project::*
};
use diesel::result::{DatabaseErrorKind, Error};
use axum::extract::Path;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse
};
use crate::establish_connection;

pub async fn show_users() -> Result<Json<Value>, StatusCode> {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();

    let results = users
        .limit(5)
        .load::<User>(connection)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let jsons: Vec<Value> = results.into_iter().map(|user| {
        json!({
            "id": user.id,
            "username": user.username,
            "email": user.email,
            "created_at": user.created_at
        })
    }).collect();

    Ok(Json(json!(jsons)))
}
