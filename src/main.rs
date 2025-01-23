mod file_operations;

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
    file_operations::check_for_conf(&filename);
    match file_operations::is_file_empty(&filename) {
        Ok(true) => {
            println!("Enter the credentials below");
            user_info = file_operations::define_userinfo(&filename);
        }
        Ok(false) => {
            println!("Reading credentials...");
            user_info = file_operations::get_userinfo(&filename);
        },
        Err(err) => println!("Error: {}", err),
    }
}
