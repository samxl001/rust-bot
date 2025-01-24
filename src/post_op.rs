use reqwest::blocking::Client;

pub fn read_posts(token: &str) {
    let user_agent = "RustRedditClient/0.1";
    let http_client = Client::new();
    let response: serde_json::Value = http_client
        .get("https://oauth.reddit.com/r/all/hot")
        .bearer_auth(token)
        .header("user-agent", user_agent)
        .send().expect("Failed to send request")
        .json().expect("Failed to get json");
    if let Some(posts) = response["data"]["children"].as_array() {
        for post in posts {
            if let Some(data) = post["data"].as_object() {
                if let Some(title) = data["title"].as_str() {
                    println!("Post title: {}", title);
                }
            }
        }
    }
}