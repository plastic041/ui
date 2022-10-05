use crate::schema::{posts, tags};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize, PartialEq)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, Serialize, Associations, PartialEq)]
#[belongs_to(Post)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub post_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub name: String,
    pub post_id: i32,
}
