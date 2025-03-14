use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = documents)]
#[diesel(belongs_to(super::Project, foreign_key = project_id))]
#[diesel(belongs_to(super::Team, foreign_key = team_id))]
#[diesel(belongs_to(super::User, foreign_key = created_by))]
pub struct Document {
    pub id: i64,
    pub title: String,
    pub content: Option<String>,
    pub project_id: Option<i64>,
    pub team_id: Option<i64>,
    pub created_by: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// POST
#[derive(Insertable)]
#[diesel(table_name = documents)]
pub struct NewDocument<'a> {
    pub title: &'a str,
    pub content: Option<&'a str>,
    pub project_id: Option<i64>,
    pub team_id: Option<i64>,
    pub created_by: i64,
}

/// PATCH
#[derive(AsChangeset)]
#[diesel(table_name = documents)]
pub struct UpdateDocument {
    pub title: Option<String>,
    pub content: Option<String>,
    pub project_id: Option<i64>,
    pub team_id: Option<i64>,
}
