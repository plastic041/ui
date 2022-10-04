use crate::schema::posts;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
}
