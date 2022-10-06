use crate::models::{Post, Tag};
use crate::schema::post_tags::dsl::{post_id, post_tags, tag_id};
use crate::schema::posts::dsl::{id, posts};
use crate::schema::tags::dsl::{name, tags};
use crate::SqlitePool;
use diesel::prelude::*;

#[tauri::command]
pub fn show_posts(filter_names: Vec<String>, state: tauri::State<SqlitePool>) -> Vec<Post> {
    let mut connection = state.get().unwrap();

    let mut tags_query_ids: Vec<i32> = Vec::new();
    for filter_name in filter_names {
        let tag = tags
            .filter(name.like(format!("{}%", filter_name)))
            .load::<Tag>(&mut connection);
        match tag {
            Ok(tag) => {
                for t in tag {
                    if !tags_query_ids.contains(&t.id) {
                        tags_query_ids.push(t.id);
                    }
                }
            }
            Err(_) => continue,
        }
    }

    println!("{:?} tags query ids", tags_query_ids);

    // diesel where AND clause
    let mut posts_query = posts.into_boxed();
    for tag_query_id in tags_query_ids {
        posts_query =
            posts_query.filter(id.eq_any(post_tags.select(post_id).filter(tag_id.eq(tag_query_id))))
    }

    posts_query.load::<Post>(&mut connection).unwrap()

    // let mut post_tags_ids: Vec<i32> = Vec::new();
    // for tag in tags_query {
    //     let post_tags = PostTag::belonging_to(&tag)
    //         .load::<PostTag>(&mut connection)
    //         .unwrap();
    //     for post_tag in post_tags {
    //         post_tags_ids.push(post_tag.post_id);
    //     }
    // }
    // println!("{:?}", post_tags_ids);

    // let mut posts_query = posts.into_boxed();
    // for post_tag_id in post_tags_ids {
    //     posts_query = posts_query.or_filter(id.eq(post_tag_id));
    // }
    // let posts_query = posts_query.load::<Post>(&mut connection).unwrap();

    // for tag in tags_query {
    //     let post_tags: Vec<PostTag> = PostTag::belonging_to(&tag)
    //         .load::<PostTag>(&mut connection)
    //         .unwrap();

    //     println!("{:?}", post_tags);
    //     for post_tag in post_tags {
    //         let post = posts
    //             .filter(id.eq(post_tag.post_id))
    //             .first::<Post>(&mut connection)
    //             .unwrap();
    //         posts_query.push(post);
    //     }
    // }

    // posts_query

    // let mut posts_ids = Vec::new();
    // for tag_id in tags_ids {
    //     let post_tags = PostTag::belonging_to(&tag_id)
    //         .load::<PostTag>(&mut connection)
    //         .unwrap();
    //     for post_tag in post_tags {
    //         posts_ids.push(post_tag.post_id);
    //     }
    // }

    // println!("{:?}", posts_ids);
}
