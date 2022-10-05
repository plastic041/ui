use crate::models::Tag;
use crate::schema::tags::dsl::*;
use crate::SqlitePool;
use diesel::prelude::*;

#[tauri::command]
pub fn show_tags(filter_name: String, state: tauri::State<SqlitePool>) -> Vec<Tag> {
    tags.filter(name.like(format!("%{}%", filter_name)))
        .load::<Tag>(&mut *state.get().unwrap())
        .expect("Error loading tags")
}
