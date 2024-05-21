use serde::{Deserialize, Serialize};
use diesel::prelude::*;

use super::schema::posts;
use super::schema::posts::dsl as posts_dsl;

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "posts"]
pub struct Posts{
    id: i32,
    title: String,
    body: String,
    published: Boolean
}

impl Posts {
    pub fn list_all() -> Vec<Self>{
        posts_dsl.load
    }
}