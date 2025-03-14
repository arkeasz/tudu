use diesel::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use crate::schema::{calendar_events, comments};

#[derive(Queryable, Identifiable)]
#[diesel(table_name = calendar_events)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct CalendarEvent {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub project_id: Option<i64>,
    pub created_by: i64,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = calendar_events)]
pub struct NewCalendarEvent<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub project_id: Option<i64>,
    pub created_by: i64,
    pub created_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = calendar_events)]
pub struct UpdateCalendarEvent<'a> {
    pub title: Option<&'a str>,
    pub description: Option<&'a str>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub project_id: Option<i64>,
}
