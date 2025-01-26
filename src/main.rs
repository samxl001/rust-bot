use rusqlite::fallible_iterator::FallibleIterator;

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
    define_user_cred(cred_filename, &mut user_info);
    setup_token(&token_filename, user_info, &mut token);
    post_op::read_posts(&token);
    let query: Vec<String> = vec![
        "Trump".to_string(),
        "Drumpf".to_string(),
        "Donald".to_string(),
        "Sweet Potato Hitler".to_string(),
        "Mango Mussolini".to_string(),
        "Cheeto-in-Chief".to_string(),
        "Agent Orange".to_string(),
        "Tangerine Tornado".to_string(),
    ];
    let posts_vec: Vec<String> = db_op::query_values("data.db3", "posts").unwrap();
    let results_vec: Vec<String> = analysis_op::search_titles(query, posts_vec);
    for results in results_vec {
        println!("{}", results);
    }
}

fn setup_token(token_filename: &String, mut user_info: UserInfo, token: &mut String) {
    file_io::check_for_file(&token_filename);
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
