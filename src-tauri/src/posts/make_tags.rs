use crate::models::NewTag;
use crate::schema::tags::dsl::*;
use crate::SqlitePool;
use diesel::RunQueryDsl;

#[tauri::command]
pub fn make_tags(state: tauri::State<SqlitePool>) {
    // make 150 tags
    let mut new_tags = Vec::new();
    for i in 1..=50 {
        let new_tag = NewTag {
            name: "a".repeat(i).to_string(),
            post_id: 0,
        };
        new_tags.push(new_tag);
    }
    for i in 1..=50 {
        let new_tag = NewTag {
            name: "b".repeat(i).to_string(),
            post_id: 1,
        };
        new_tags.push(new_tag);
    }
    for i in 1..=50 {
        let new_tag = NewTag {
            name: "c".repeat(i).to_string(),
            post_id: 2,
        };
        new_tags.push(new_tag);
    }
    for i in 1..=50 {
        let new_tag = NewTag {
            name: "d".repeat(i).to_string(),
            post_id: 3,
        };
        new_tags.push(new_tag);
    }
    for i in 1..=50 {
        let new_tag = NewTag {
            name: "e".repeat(i).to_string(),
            post_id: 4,
        };
        new_tags.push(new_tag);
    }
    diesel::insert_into(tags)
        .values(&new_tags)
        .execute(&mut *state.get().unwrap())
        .expect("Error saving new tag");
}
