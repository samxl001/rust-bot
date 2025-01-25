use super::db_op;
use reqwest::blocking::Client;

pub fn read_posts(token: &str) {
    let user_agent = "RustRedditClient/0.1";
    let http_client = Client::new();
    let mut post_vec: Vec<&str> = Vec::new();
    let db_filename = String::from("data.db3");
    let response: serde_json::Value = http_client
        .get("https://oauth.reddit.com/r/all/hot?limit=100")
        .bearer_auth(token)
        .header("user-agent", user_agent)
        .send()
        .expect("Failed to send request")
        .json()
        .expect("Failed to get json");

    if let Some(posts) = response["data"]["children"].as_array() {
        for post in posts {
            if let Some(data) = post["data"].as_object() {
                if let Some(title) = data["title"].as_str() {
                    //println!("Post title: {}", title);
                    if !post_vec.contains(&title) {
                        post_vec.push(title);
                    }
                }
            }
        }
    }
    // for post in &post_vec {
    //     println!("{}", post);
    // }
    //println!("{}", post_vec.len());
    db_op::populate_table(&db_filename, post_vec);
    db_op::query_rows(&db_filename);
    //db_op::delete_table(&db_filename,"posts");
}
