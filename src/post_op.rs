use super::{authentication, db_op, file_io, UserInfo};
use reqwest::blocking::Client;
use serde_json::Value;
use std::error::Error;

fn fetch_reddit_posts(token: &str, user_agent: &str) -> Result<Value, Box<dyn Error>> {
    authentication::establish_connection_success();
    let http_client = Client::new();

    let response = http_client
        .get("https://oauth.reddit.com/r/all/hot?limit=100")
        .bearer_auth(token)
        .header("user-agent", user_agent)
        .send()?;

    if response.status().is_success() {
        Ok(response.json()?)
    } else {
        Err(format!("Non-success status code returned: {}", response.status()).into())
    }
}

pub fn process_reddit_posts(token: &mut str) {
    let mut post_vec: Vec<&str> = Vec::new();
    let db_filename = String::from("data.db3");
    let user_agent = "RustRedditClient/0.1";
    match fetch_reddit_posts(token, user_agent) {
        Ok(json) => {
            if let Some(posts) = json["data"]["children"].as_array() {
                for post in posts {
                    if let Some(data) = post["data"].as_object() {
                        if let Some(title) = data["title"].as_str() {
                            if !post_vec.contains(&title) {
                                post_vec.push(title);
                            }
                        }
                    }
                }
            }
            db_op::populate_table(&db_filename, "posts", post_vec)
                .expect("Failed to populate table");
        }
        Err(e) => {
            println!("Failed to fetch or process posts: {}", e);
            if e.to_string().contains("401") {
                //TODO: setup a way to renew token dynamically
                println!("Retrying with a new token...");
            } else {
                println!("Unhandled error. Exiting.");
            }
        }
    }
}
