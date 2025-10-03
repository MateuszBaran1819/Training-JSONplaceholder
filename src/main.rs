use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
struct Post {
    userId: i32, //warning in the compilation - name came from jsonplacecholder
    id: i32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let post_val: Vec<Post> = reqwest::get("https://jsonplaceholder.typicode.com/posts")
        .await?
        .json()
        .await?;
    let _ = save_post(post_val);
    //println!("Pobrano {} postow", Post_val.len());

    Ok(())
}

fn save_post(tab: Vec<Post>) -> Result<(), std::io::Error> {
    let dir = "JSON_files";

    for post in &tab {
        let json = serde_json::to_string_pretty(post).unwrap();
        let file_name = format!("{}/post_{}.json", dir, post.id);
        let mut file = File::create(file_name)?;
        file.write_all(json.as_bytes())?;
    }
    Ok(())
}
