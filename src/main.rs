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
    file_io::check_for_file(&token_filename);
    match file_io::is_file_empty(&token_filename) {
        Ok(true) => {
            println!("Getting token to authenticate",);
            token = authentication::get_token(&token_filename, user_info);
        }
        Ok(false) => {
            println!("Using token to authenticate");
            token = authentication::use_token(&token_filename);
        }
        Err(err) => println!("Other error occurred: {}", err),
    }
    post_op::read_posts(&token);
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
