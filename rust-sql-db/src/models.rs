// Postgres DB
#[derive(Queryable, Debug, Deserialize, Serialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

// JSON Rest
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InputPost {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub posts: Vec<Post>,
}

