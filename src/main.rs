mod analysis_op;
mod authentication;
mod db_op;
mod file_io;
mod post_op;

struct UserInfo {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
}
impl UserInfo {
    fn new() -> Self {
        UserInfo {
            client_id: String::new(),
            client_secret: String::new(),
            username: String::new(),
            password: String::new(),
        }
    }
}

fn main() {
    let cred_filename = String::from("credentials.txt");
    let token_filename = String::from("token.txt");
    let database_filename = String::from("data.db3");
    let mut user_info = UserInfo::new();
    let mut token = String::new();
    let keyword_filename = String::from("keywords.json");
    define_user_cred(cred_filename, &mut user_info);
    setup_token(&token_filename, user_info, &mut token);
    post_op::process_reddit_posts(&mut token);
    let posts_vec: Vec<String> = db_op::query_values(&database_filename, "posts").unwrap();
    let query = file_io::load_queries_from_json(&keyword_filename);
    let mut keywords: Vec<String> = Vec::new();
    let mut mentions: Vec<i64> = Vec::new();
    match query {
        Ok(map) => {
            // Directly use the map instead of borrowing it with &
            for (key, value) in map { // Move out of the map directly
                println!("Key: {}", key);
                keywords.push(key.clone()); // Convert String to &str
                db_op::update_mentions_table(&database_filename, key).expect("Failed to update mentions table");
                let result = analysis_op::search_titles(value.to_vec(), &posts_vec);
                if result.is_empty() {
                    mentions.push(0);
                }
                else { 
                    mentions.push(result.len() as i64);
                }
                
            }
        }
        Err(e) => {
            // Handle the error case
            eprintln!("Error: {}", e);
        }
    }
    db_op::update_field_values(&database_filename, keywords, mentions).expect("Failed to update mentions table values");
    
    
}

fn setup_token(token_filename: &String, user_info: UserInfo, token: &mut String) {
    file_io::check_for_file(&token_filename);
    authentication::establish_connection_success();
    match file_io::is_file_empty(&token_filename) {
        Ok(true) => {
            println!("Getting token to authenticate",);
            *token = authentication::get_token(&token_filename, &user_info);
        }
        Ok(false) => {
            println!("Using token to authenticate");
            *token = authentication::use_token(&token_filename);
        }
        Err(err) => println!("Other error occurred: {}", err),
    }
}

fn define_user_cred(cred_filename: String, user_info: &mut UserInfo) {
    file_io::check_for_file(&cred_filename);
    match file_io::is_file_empty(&cred_filename) {
        Ok(true) => {
            println!("Enter the credentials below");
            *user_info = file_io::define_userinfo(&cred_filename);
        }
        Ok(false) => {
            println!("Reading credentials...");
            *user_info = file_io::get_userinfo(&cred_filename);
        }
        Err(err) => println!("Other error occurred: {}", err),
    }
}
