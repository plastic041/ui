use crate::models::{Post, PostTag, Tag};
use crate::schema::posts::dsl::{id, posts, title};
use crate::schema::tags::dsl::{name, tags};
use crate::SqlitePool;
use diesel::prelude::*;
use diesel::BelongingToDsl;

#[tauri::command]
pub fn show_posts(filter_names: Vec<String>, state: tauri::State<SqlitePool>) -> Vec<Post> {
    let mut tags_query = tags.into_boxed();
    for filter_name in filter_names {
        tags_query = tags_query.filter(name.like(format!("%{}%", filter_name)));
    }
    let tags_found: Vec<Tag> = tags_query
        .load(&mut *state.get().unwrap())
        .expect("Error loading tags");

    println!("{:?}", tags_found);

    let res: Vec<Post> = PostTag::belonging_to(&tags_found)
        .inner_join(posts)
        .select((id, title))
        .load(&mut *state.get().unwrap())
        .expect("Error loading posts");

    res

    // let mut posts_query = posts.into_boxed();
    // for tag in tags_found {
    //     posts_query = posts_query.filter(id.eq(tag.id));
    // }

    // // get ids of tags found
    // let mut tag_ids = Vec::new();
    // for tag in tags_found {
    //     // handle duplicates
    //     if !tag_ids.contains(&tag.post_id) {
    //         tag_ids.push(tag.post_id);
    //     }
    // }

    // posts
    //     .filter(id.eq_any(tag_ids))
    //     .load::<Post>(&mut *state.get().unwrap())
    //     .expect("Error loading posts")
}
