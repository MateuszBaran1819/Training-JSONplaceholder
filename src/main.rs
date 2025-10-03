use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
struct Post {
    userId: i32,
    id: i32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Post: Vec<Post> = reqwest::get("https://jsonplaceholder.typicode.com/posts")
        .await?
        .json()
        .await?;
    println!("Pobrano {} postow", Post.len());
    Ok(())
}
