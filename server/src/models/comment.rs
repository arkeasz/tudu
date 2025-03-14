use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = comments)]
pub struct Comment {
    pub id: i64,
    pub task_id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment<'a> {
    pub task_id: i64,
    pub user_id: i64,
    pub content: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = comments)]
pub struct UpdateComment<'a> {
    pub content: Option<&'a str>,
}
