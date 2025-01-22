mod file_operations;

struct Userinfo {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
}

fn main() {
    let filename = String::from("info.txt");
    file_operations::check_for_conf(&filename);
    match file_operations::is_file_empty(&filename) {
        Ok(true) => println!("Enter the credentials below"),
        Ok(false) => println!("Logging in..."),
        Err(err) => println!("Error: {}", err)
    }
}
