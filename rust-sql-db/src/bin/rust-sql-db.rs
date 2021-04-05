extern crate rust_sql_db;
extern crate diesel;

use self::rust_sql_db::*;
use self::models::*;
use self::diesel::prelude::*;
use serde::Serialize;
use warp::Filter;
use warp::Reply;

#[tokio::main]
async fn main() {
  println!("Rust PosgreSQL app started at port 3002.");
  
  let get_endpoint = warp::path("posts")
    .map(|| warp::reply::json(&get_response()));

  let post_endpoint = warp::path("new")
    .and(warp::post())
    .and(warp::body::json())
    .map(|content| post_reply(content));

  let routes = get_endpoint.or(post_endpoint);
  warp::serve(routes).run(([0, 0, 0, 0], 3002)).await;
}

fn get_response() -> Response{
  use rust_sql_db::schema::posts::dsl::*;

  let connection = establish_connection();
  let results = posts
    .load::<Post>(&connection)
    .expect("Error loading posts");

  Response{posts: results}
}

fn post_reply(content: InputPost) -> String {
  use rust_sql_db::schema::posts::dsl::*;
  println!("{:?}", content);

  let connection = establish_connection();
  create_post(&connection, &content.title, &content.body);

  format!("Accepted new post: {:?}", content)
}