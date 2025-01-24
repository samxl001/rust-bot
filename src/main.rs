mod file_io;
mod authentication;
mod post_op;

struct Userinfo {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
}
impl Userinfo {
    fn new() -> Self {
        Userinfo {
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
    let mut user_info = Userinfo::new();
    define_user_cred(cred_filename, &mut user_info);
    let token = authentication::get_token(user_info);
    post_op::read_posts(&token);
}

fn define_user_cred(cred_filename: String, user_info: &mut Userinfo) {
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
        Err(err) => println!("Error: {}", err),
    }
}
