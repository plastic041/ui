use crate::models::NewPost;
use crate::schema::posts::dsl::*;
use crate::SqlitePool;
use diesel::RunQueryDsl;

#[tauri::command]
pub fn make_posts(state: tauri::State<SqlitePool>) {
    // make 150 posts
    for i in 1..=150 {
        let new_post = NewPost {
            title: "p".repeat(i).to_string(),
        };
        diesel::insert_into(posts)
            .values(&new_post)
            .execute(&mut *state.get().unwrap())
            .expect("Error saving new post");
    }
}
