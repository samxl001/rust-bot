mod file_io;
mod authentication;

struct Userinfo {
    client_id: String,
    client_secret: String,
}
impl Userinfo {
    fn new() -> Self {
        Userinfo {
            client_id: String::new(),
            client_secret: String::new(),
            //auth_url: String::from("https://www.reddit.com/api/v1/authorize"),
            //token_url: String::from("https://www.reddit.com/api/v1/access_token")
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
}
