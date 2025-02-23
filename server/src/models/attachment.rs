use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::attachments;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = attachments)]
pub struct Attachment {
    pub id: i64,
    pub task_id: Option<i64>,
    pub document_id: Option<i64>,
    pub file_path: String,
    pub uploaded_by: i64,
    pub uploaded_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = attachments)]
pub struct NewAttachment {
    pub task_id: Option<i64>,
    pub document_id: Option<i64>,
    pub file_path: String,
    pub uploaded_by: i64,
    pub uploaded_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = attachments)]
pub struct UpdateAttachment {
    pub task_id: Option<i64>,
    pub document_id: Option<i64>,
    pub file_path: Option<String>,
}
