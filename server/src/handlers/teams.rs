use crate::models::team::*;
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
pub struct NewTeamRequest {
    name: String,
    alias: String,
    description: String,
}

#[derive(Debug, Serialize)]
pub struct TeamResponse {
    id: i64,
    name: String,
    description: String,
    alias: String
}


#[derive(Deserialize)]
pub struct UpdateTeamRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub alias: Option<String>
}

pub async fn show_teams() -> Result<Json<Value>, StatusCode> {
    use crate::schema::teams::dsl::*;
    let connection = &mut establish_connection();

    let results = teams
        .limit(5)
        .load::<Team>(connection)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let jsons: Vec<Value> = results.into_iter().map(|tim| {
        json!({
            "id": tim.id,
            "name": tim.name,
            "description": tim.description,
            "created_at": tim.created_at,
            "alias": tim.alias,
        })
    }).collect();

    Ok(Json(json!(jsons)))
}

pub async fn create_team(Json(payload): Json<NewTeamRequest>) -> impl IntoResponse {
    use crate::schema::teams;

    let connection = &mut establish_connection();

    let new_team = NewTeam {
        name: &payload.name,
        description: Some(&payload.description),
        alias: Some(&payload.alias)
    };

    let result = connection.transaction(|conn| {
        diesel::insert_into(teams::table)
            .values(&new_team)
            .execute(conn)?;

        teams::table
            .order(teams::id.desc())
            .select(Team::as_select())
            .first(conn)
    });

    match result {
        Ok(team) => (
            StatusCode::CREATED,
            Json(TeamResponse {
                id: team.id as i64,
                name: team.name,
                description: team.description.unwrap_or("".to_string()),
                alias: team.alias.unwrap_or("".to_string()),
            }),
        )
        .into_response(),

        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => (
            StatusCode::CONFLICT,
            Json(json!({ "error": "The alias is already in use" })),
        )
        .into_response(),

        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error" })),
        )
        .into_response(),
    }
}

pub async fn update_team(
    Path(team_id): Path<i64>,
    Json(payload): Json<UpdateTeamRequest>,
) -> impl IntoResponse {
    use crate::schema::teams::dsl::*;

    let connection = &mut establish_connection();

    let update_data = UpdateTeam {
        name: payload.name,
        description: payload.description,
        alias: payload.alias,
    };

    let result = diesel::update(teams.find(team_id))
        .set(&update_data)
        .execute(connection);

    match result {
        Ok(0) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Team not found" })),
        )
        .into_response(),

        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Team updated successfully" })),
        )
        .into_response(),

        Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _)) => (
            StatusCode::CONFLICT,
            Json(json!({ "error": "alias already exists" })),
        )
        .into_response(),

        Err(e) => {
            eprintln!("Database error: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {:?}", e) })),
            )
            .into_response()
        }
    }
}


pub async fn delete_team(
    Path(team_id): Path<i64>
) -> impl IntoResponse {
    use crate::schema::teams::dsl::*;

    let connection = &mut establish_connection();

    let result = diesel::delete(teams.filter(id.eq(team_id)))
        .execute(connection);

    match result {
        Ok(0) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Team not found" }))
        ).into_response(),

        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Team deleted successfully" }))
        ).into_response(),

        Err(e) => {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {:?}", e) })))
            .into_response()
        }
    }
}
