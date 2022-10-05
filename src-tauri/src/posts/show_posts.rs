// use crate::models::Post;
// use crate::schema::posts::dsl::*;
// use crate::SqlitePool;
// use diesel::prelude::*;

// #[tauri::command]
// pub fn show_posts(filter_title: String, state: tauri::State<SqlitePool>) -> Vec<Post> {
//     posts
//         .filter(title.like(format!("%{}%", filter_title)))
//         // .limit(5)
//         .load::<Post>(&mut *state.get().unwrap())
//         .expect("Error loading posts")
// }

use crate::models::{Post, Tag};
use crate::schema::posts::dsl::{id, posts};
use crate::schema::tags::dsl::{name, tags};
use crate::SqlitePool;
use diesel::prelude::*;

#[tauri::command]
pub fn show_posts(filter_name: String, state: tauri::State<SqlitePool>) -> Vec<Post> {
    let tags_found = tags
        .filter(name.like(format!("%{}%", filter_name)))
        .load::<Tag>(&mut *state.get().unwrap())
        .expect("Error loading tags");

    // get ids of tags found
    let mut tag_ids = Vec::new();
    for tag in tags_found {
        // handle duplicates
        if !tag_ids.contains(&tag.post_id) {
            tag_ids.push(tag.post_id);
        }
    }

    posts
        .filter(id.eq_any(tag_ids))
        // .filter(title.like(format!("%{}%", filter_title)))
        .load::<Post>(&mut *state.get().unwrap())
        .expect("Error loading posts")
}
