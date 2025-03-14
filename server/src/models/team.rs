use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::*;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

/// POST
#[derive(Insertable)]
#[diesel(table_name = teams)]
pub struct NewTeam<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
}

/// PATCH
#[derive(AsChangeset)]
#[diesel(table_name = teams)]
pub struct UpdateTeam {
    pub name: Option<String>,
    pub description: Option<String>,
}
