use crate::models::*;
use diesel::prelude::*;
use serde_json::{Value, json};
use axum::{
    extract::Path,
    Json
};
use diesel::sql_types::Json as OtherJson;
use crate::establish_connection;

pub async fn show_posts() -> Json<Value> {
    use crate::schema::posts::dsl::*;
    let connection = &mut establish_connection();

    let results = posts
    .filter(published.eq(true))
    .limit(5)
    .select(Post::as_select())
    .load(connection)
    .expect("Error loading posts");

    let jsons: Vec<Value> = results.into_iter().map(|post| {
        json!({
            "id": post.id,
            "title": post.title,
            "body": post.body,
            "published": post.published,
        })
    }).collect();

    Json(json!(jsons))
}

pub async fn get_post(Path(id): Path<u32>) -> Json<Value> {
    use crate::schema::posts::dsl::posts;
    let connection = &mut establish_connection();
    let post = posts
        .find(id as i32)
        .select(Post::as_select())
        .first(connection)
        .optional();
    match post {
        Ok(Some(post)) => {
            Json(json!({
                "id": post.id,
                "title": post.title,
                "body": post.body
            }))
        },
        Ok(None) => {
            Json(json!({"message": format!("Unable to find post {}", id)}))
        },
        Err(_) =>{
            Json(json!({"message": format!("An erro ocurred while feching post {}", id)}))
        }
    }
}
