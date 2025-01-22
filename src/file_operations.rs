use std::{fs, io};
pub fn check_for_conf(filename: &str) {
    match fs::File::open(filename) {
        Ok(_) => {println!("File found")}
        Err(_) => {
            println!("The file does not exist. Creating file...");
            match fs::File::create(filename) {
                Ok(_) => {println!("File is created")}
                Err(err) => {
                    println!("Could not create file");
                    println!("Error: {}", err);
                }
            }
        }
    }
    
}
pub fn is_file_empty(filename: &str) -> Result<bool, std::io::Error> {
    let file_metadata = fs::metadata(filename)?;
    Ok(file_metadata.len() == 0)
}
