use std::time::Duration;

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
    let keyword_result = file_io::get_vec_from_json(&keyword_filename);
    let mut query_vec: Vec<String> = Vec::new();
    match keyword_result {
        Ok(keyword_vec) => {
            for keyword in keyword_vec {
                for word in keyword {
                    query_vec.push(word.to_string()); 
                }
            }
        }
        Err(e) => println!("Could not read keywords file: {}", e),
    }
    define_user_cred(cred_filename, &mut user_info);
    setup_token(&token_filename, user_info, &mut token);
    post_op::read_titles(&token);
    let posts_vec: Vec<String> = db_op::query_values("data.db3", "posts").unwrap();
    let results_vec: Vec<String> = analysis_op::search_titles(query_vec, posts_vec);
    for results in results_vec {
        println!("{}", results);
    }
}

fn setup_token(token_filename: &String, user_info: UserInfo, token: &mut String) {
    file_io::check_for_file(&token_filename);
    loop {
        match authentication::check_internet() { 
            Ok(()) => {
                println!("Internet access is found");
                break;
            }
            Err(e) => {
                println!("{} Retrying", e);
                std::thread::sleep(Duration::from_secs(5));
            }
        }
    }
    match file_io::is_file_empty(&token_filename) {
        Ok(true) => {
            println!("Getting token to authenticate", );
            *token = authentication::get_token(&token_filename, user_info);
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
