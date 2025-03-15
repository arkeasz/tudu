use crate::models::{
    user::*,
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
pub struct NewUserRequest {
    email: String,
    password_hash: String,
    username: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    id: i64,
    email: String,
    username: String,
}


#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
}

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

pub async fn create_user(Json(payload): Json<NewUserRequest>) -> impl IntoResponse {
    use crate::schema::users;

    let connection = &mut establish_connection();

    let new_user = NewUser {
        email: &payload.email,
        password_hash: &payload.password_hash,
        username: &payload.username,
    };

    let result = connection.transaction(|conn| {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        users::table
            .order(users::id.desc())
            .select(User::as_select())
            .first(conn)
    });

    match result {
        Ok(user) => (
            StatusCode::CREATED,
            Json(UserResponse {
                id: user.id as i64,
                email: user.email,
                username: user.username,
            }),
        )
        .into_response(),

        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => (
            StatusCode::CONFLICT,
            Json(json!({ "error": "The email or username is already in use" })),
        )
        .into_response(),

        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error" })),
        )
        .into_response(),
    }
}

pub async fn update_user(
    Path(user_id): Path<i64>,
    Json(payload): Json<UpdateUserRequest>,
) -> impl IntoResponse {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let update_fields = UpdateUser {
        username: &payload.username.unwrap_or("".to_string()),
        email: &payload.email.unwrap_or("".to_string()),
        password_hash: &payload.password_hash.unwrap_or("".to_string()),
    };

    let result = diesel::update(users.find(user_id as i64))
        .set(&update_fields)
        .execute(connection);

    match result {
        Ok(0) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" }))
        ).into_response(),

        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "User updated successfully" }))
        ).into_response(),

        Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => (
            StatusCode::CONFLICT,
            Json(json!({ "error": "Username already exists" }))
        ).into_response(),

        Err(e) => {
            eprintln!("Database error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {:?}", e) })))
            .into_response()
        }
    }
}

pub async fn delete_user(Path(user_id): Path<i64>) -> impl IntoResponse {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();

    let result = diesel::delete(users.filter(id.eq(user_id)))
        .execute(connection);

    match result {
        Ok(0) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "User not found" }))
        ).into_response(),

        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "User deleted successfully" }))
        ).into_response(),

        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {:?}", e) })))
            .into_response()
        }
    }
}
