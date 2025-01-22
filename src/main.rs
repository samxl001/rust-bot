use std::{fs, io};
struct Userinfo {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
}

fn main() {
    let user_file = String::from("info.txt");
    check_for_conf(user_file);
}
fn check_for_conf(user_file: String) {
    match fs::File::open(&user_file) {
        Ok(_) => {println!("File found")}
        Err(_) => {
            println!("The file does not exist. Creating file...");
            match fs::File::create(&user_file) {
                Ok(_) => {println!("File is created")}
                Err(err) => {
                    println!("Could not create file");
                    println!("Error: {}", err);
                }
            }
        }
    }
    
}