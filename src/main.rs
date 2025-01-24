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
    let filename = String::from("info.txt");
    let mut user_info = Userinfo::new();
    file_io::check_for_conf(&filename);
    match file_io::is_file_empty(&filename) {
        Ok(true) => {
            println!("Enter the credentials below");
            user_info = file_io::define_userinfo(&filename);
        }
        Ok(false) => {
            println!("Reading credentials...");
            user_info = file_io::get_userinfo(&filename);
        }
        Err(err) => println!("Error: {}", err),
    }
    let token = authentication::get_token(user_info);
    post_op::read_posts(&token);
}
