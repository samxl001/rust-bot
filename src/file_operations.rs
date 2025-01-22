use std::fs;
pub fn check_for_conf(user_file: String) {
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