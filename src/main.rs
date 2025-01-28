
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
    let mut user_info = UserInfo::new();
    let mut token = String::new();
    let keyword_filename = String::from("keywords.json");
    define_user_cred(cred_filename, &mut user_info);
    setup_token(&token_filename, user_info, &mut token);
    post_op::process_reddit_posts(&mut token);
    let posts_vec: Vec<String> = db_op::query_values("data.db3", "posts").unwrap();
    let query = file_io::load_queries_from_json(&keyword_filename);
    match query {
        Ok(map) => {
            // Iterate through the HashMap if it's Ok
            for (key, value) in &map {
                println!("Key: {}", key);
                let result = analysis_op::search_titles(value.to_vec(), &posts_vec);
                //println!("Result: {:?}", result);
            }
        }
        Err(e) => {
            // Handle the error case
            eprintln!("Error: {}", e);
        }
    }
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
