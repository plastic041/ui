use crate::models::NewPost;
use crate::models::NewPostTag;
use crate::models::NewTag;
use crate::models::Tag;
use crate::schema::post_tags::dsl as post_tags_dsl;
use crate::schema::posts::dsl as posts_dsl;
use crate::schema::tags::dsl as tags_dsl;
use crate::SqlitePool;
use diesel::prelude::*;

fn make_post(state: &mut SqlitePool) {
    let mut connection = state.get().unwrap();

    let post_title = "batman".to_string();
    let post_tags = vec!["hero".to_string(), "male".to_string()];

    let new_post = NewPost { title: post_title };

    let post_id = diesel::insert_into(posts_dsl::posts)
        .values(&new_post)
        .returning(posts_dsl::id)
        .get_result::<i32>(&mut connection)
        .expect("Error saving new post");

    for tag_name in post_tags {
        let tag = tags_dsl::tags
            .filter(tags_dsl::name.eq(&tag_name))
            .first::<Tag>(&mut connection);
        match tag {
            Ok(tag) => {
                let new_post_tag = NewPostTag {
                    post_id,
                    tag_id: tag.id,
                };
                diesel::insert_into(post_tags_dsl::post_tags)
                    .values(&new_post_tag)
                    .execute(&mut connection)
                    .unwrap();
            }
            Err(_) => {
                let new_tag = NewTag { name: tag_name };
                let tag_id = diesel::insert_into(tags_dsl::tags)
                    .values(&new_tag)
                    .returning(tags_dsl::id)
                    .get_result::<i32>(&mut connection)
                    .unwrap();
                let new_post_tag = NewPostTag { post_id, tag_id };
                diesel::insert_into(post_tags_dsl::post_tags)
                    .values(&new_post_tag)
                    .execute(&mut connection)
                    .unwrap();
            }
        }
    }
}

#[tauri::command]
pub fn make_post_tags(state: tauri::State<SqlitePool>) {
    // make 150 posts
    for i in 1..=150 {
        let new_post = NewPost {
            title: "p".repeat(i).to_string(),
        };
        diesel::insert_into(posts_dsl::posts)
            .values(&new_post)
            .execute(&mut *state.get().unwrap())
            .expect("Error saving new post");
    }
}
