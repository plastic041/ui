use crate::schema::{post_tags, posts, tags};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize, Identifiable)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, Serialize, Identifiable, Debug)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Queryable, Serialize, Associations)]
#[diesel(belongs_to(Post, foreign_key = post_id))]
#[diesel(belongs_to(Tag, foreign_key = tag_id))]
pub struct PostTag {
    pub id: i32,
    pub post_id: i32,
    pub tag_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = post_tags)]
pub struct NewPostTag {
    pub post_id: i32,
    pub tag_id: i32,
}

#[derive(Queryable, Serialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub name: String,
}
