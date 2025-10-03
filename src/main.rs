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

/// main function to download post data from jsonplaceholder server.
/// Requirements: read all posts from jsonplaceholder and write all
/// information to .json file
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
///function to read vector which hold the data from server
/// in the for loop we will create files as many as is in the buffer
/// file name: <id_posta>.json
/// each files are placed in the JSON_files folder
fn save_post(tab: Vec<Post>) -> Result<(), std::io::Error> {
    let dir = "JSON_files";

    for post in &tab {
        let json = serde_json::to_string_pretty(post).unwrap();
        let file_name = format!("{}/{}.json", dir, post.id);
        let mut file = File::create(file_name)?;
        file.write_all(json.as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    fn cleanup_dir(dir: &str) {
        //function to clear directory
        if Path::new(dir).exists() {
            fs::remove_dir_all(dir).unwrap();
        }
        fs::create_dir_all(dir).unwrap();
    }

    #[test]
    fn test_save_post_creates_files() {
        let dir = "JSON_files";
        cleanup_dir(dir); // we will clean up the directory

        // created example posts
        let posts = vec![
            Post {
                user_id: 1,
                id: 101,
                title: "Test1".to_string(),
                body: "Body1".to_string(),
            },
            Post {
                user_id: 2,
                id: 102,
                title: "Test2".to_string(),
                body: "Body2".to_string(),
            },
        ];

        // executed our function
        save_post(posts.clone()).unwrap();

        // we will check that files was created
        for post in posts {
            let file_path = format!("{}/{}.json", dir, post.id);
            assert!(
                Path::new(&file_path).exists(),
                "Plik {} powinien istnieć",
                file_path
            );

            // we will check what is in the file
            let content = fs::read_to_string(file_path).unwrap();
            let deserialized: Post = serde_json::from_str(&content).unwrap();
            assert_eq!(deserialized.id, post.id);
            assert_eq!(deserialized.title, post.title);
        }
    }
}
