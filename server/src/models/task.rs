use diesel::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use crate::schema::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = tasks)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Task {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub assigned_to: Option<i64>,
    pub due_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
}

/// POST
#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask<'a> {
    pub project_id: i64,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub status: &'a str,
    pub assigned_to: Option<i64>,
    pub due_date: Option<NaiveDate>,
}

/// PATCH
#[derive(AsChangeset)]
#[diesel(table_name = tasks)]
pub struct UpdateTask {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub assigned_to: Option<i64>,
    pub due_date: Option<NaiveDate>,
}
