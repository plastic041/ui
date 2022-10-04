use crate::models::Post;
use crate::schema::posts::dsl::*;
use crate::SqlitePool;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

#[tauri::command]
pub fn show_posts(filter_title: String, state: tauri::State<SqlitePool>) -> Vec<Post> {
    posts
        .filter(title.like(format!("%{}%", filter_title)))
        .limit(5)
        .load::<Post>(&mut *state.get().unwrap())
        .expect("Error loading posts")
}
