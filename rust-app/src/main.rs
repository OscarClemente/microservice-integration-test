use std::io::prelude::*;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Post {
    id: i32,
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct NewPost {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
struct Posts {
    posts: Vec<Post>,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust web app!")
}

#[get("/hello_go")]
async fn hello_go() -> impl Responder {
    let mut rust_app_content = "Test Rust web app asks Go web app: ".to_string();
    let mut go_app_res = reqwest::blocking::get("http://go-web-app:3000").expect("Error connecting with go web app");
    let mut go_app_content = String::new();
    go_app_res.read_to_string(&mut go_app_content).unwrap();
    rust_app_content.push_str(&go_app_content);

    HttpResponse::Ok().body(rust_app_content)
}

#[get("/posts")]
async fn posts() -> impl Responder {
    let mut post_res = reqwest::blocking::get("http://rust-sql-db:3002/posts").unwrap();
    let mut post_content = String::new();
    post_res.read_to_string(&mut post_content).unwrap();
    let result: Value = serde_json::from_str(&post_content).unwrap();

    HttpResponse::Ok().json(result)
}

#[post("/new_post")]
async fn new_post(post: web::Json<NewPost>) -> impl Responder {
    let mut map = HashMap::new();
    map.insert("title", post.title.clone());
    map.insert("body", post.body.clone());

    let client = reqwest::blocking::Client::new();
    let mut res = client.post("http://rust-sql-db:3002/new")
        .json(&map)
        .send()
        .unwrap();
    
    let mut post_res = String::new();
    res.read_to_string(&mut post_res).unwrap();

    HttpResponse::Ok().body(format!("Great!: {}", post_res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Rust web app!");

    HttpServer::new(|| {
        App::new()
            .service(hello_go)
            .service(hello)
            .service(posts)
            .service(new_post)
    })
    .bind("0.0.0.0:3001")?
    .run()
    .await
}
