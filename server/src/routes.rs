use axum::{routing::*, Router};
use crate::handlers::{
    user::*,
    project::*,
    teams::*
};

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", get(show_users))
        .route("/users/register", post(register_user))
        .route("/users/update/{id}", patch(update_user))
        .route("/users/delete/{id}", delete(delete_user))
        .route("/users/login", post(login_user))
}

pub fn project_routes() -> Router {
    Router::new().route("/projects", get(show_projects))
}

pub fn team_routes() -> Router {
    Router::new()
        .route("/teams", get(show_teams))
        .route("/teams/create", post(create_team))
        .route("/teams/update/{id}", patch(update_team))
        .route("/teams/delete/{id}", patch(delete_team))
}
