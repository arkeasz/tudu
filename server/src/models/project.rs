use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = projects)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i64,
    pub created_at: NaiveDateTime,
    pub team_id: Option<i64>,
}

/// POST
#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub owner_id: i64,
    pub team_id: Option<i64>,
}

/// PATCH
#[derive(AsChangeset)]
#[diesel(table_name = projects)]
pub struct UpdateUser<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub owner_id: i64,
    pub team_id: Option<i64>,
}
