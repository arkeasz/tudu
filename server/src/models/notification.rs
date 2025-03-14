use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = notifications)]
#[diesel(belongs_to(super::User, foreign_key = user_id))]
pub struct Notification {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
    pub is_read: Option<bool>,
    pub created_at: NaiveDateTime,
}

/// POST
#[derive(Insertable)]
#[diesel(table_name = notifications)]
pub struct NewNotification<'a> {
    pub user_id: i64,
    pub content: &'a str,
    pub is_read: Option<bool>,
}

/// PATCH
#[derive(AsChangeset)]
#[diesel(table_name = notifications)]
pub struct UpdateNotification {
    pub is_read: Option<bool>,
}
