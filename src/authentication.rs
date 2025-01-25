use super::file_io;
use super::UserInfo;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: Option<String>, // Token may be absent in error responses
    token_type: Option<String>,
    expires_in: Option<u64>,
    scope: Option<String>,
    error: Option<String>,   // Handles error responses
    message: Option<String>, // Additional error information
}

pub fn get_token(filename: &str, user_info: UserInfo) -> String {
    let client_id = &user_info.client_id;
    let client_secret = &user_info.client_secret;
    let username = &user_info.username;
    let password = &user_info.password;
    let mut info_vec: Vec<String> = Vec::new();
    let client = Client::new();
    let user_agent = "RustRedditClient/0.1";

    let response = client
        .post("https://www.reddit.com/api/v1/access_token")
        .header("User-Agent", user_agent)
        .basic_auth(client_id, Some(client_secret))
        .form(&[
            ("grant_type", "password"),
            ("username", username),
            ("password", password),
        ])
        .send()
        .expect("Failed to send request");

    let response_text = response.text().expect("Failed to read response text");
    //println!("Raw response: {}", response_text);

    let parsed_response: TokenResponse =
        serde_json::from_str(&response_text).expect("Failed to parse response");

    if let Some(access_token) = parsed_response.access_token {
        //println!("Access token: {}", access_token);
        info_vec.push(access_token.clone());
        file_io::write_to_file(filename, &info_vec);
        access_token
    } else {
        println!("Error: {:?}", parsed_response.error);
        panic!("Failed to obtain access token");
    }
}
pub fn use_token(filename: &str) -> String {
    let mut info_vec: Vec<String> = Vec::new();
    let mut access_token = String::new();
    file_io::get_data(filename, &mut info_vec);
    access_token = info_vec.remove(0);
    access_token
}
