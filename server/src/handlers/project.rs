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

#[derive(Debug, Deserialize)]
pub struct NewProjectRequest {
    email: String,
    password_hash: String,
    username: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    id: i64,
    email: String,
    username: String,
}

#[derive(Deserialize)]
pub struct UpdateProjectRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
}

pub async fn show_projects() -> Result<Json<Value>, StatusCode> {
    use crate::schema::projects::dsl::*;
    let connection = &mut establish_connection();

    let results = projects
        .limit(5)
        .load::<Project>(connection)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let jsons: Vec<Value> = results.into_iter().map(|pro| {
        json!({
            "id": pro.id,
            "name": pro.name,
            "description": pro.description,
            "owner_id": pro.owner_id,
            "created_at": pro.created_at,
            "team_id": pro.team_id
        })
    }).collect();

    Ok(Json(json!(jsons)))
}
