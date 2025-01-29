use super::UserInfo;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::{
    fs,
    io::{self, BufRead, Error, ErrorKind, Write},
};

#[derive(Deserialize)]
struct Queries {
    queries: HashMap<String, Vec<String>>,
}

pub fn check_for_file(filename: &str) {
    match fs::File::open(filename) {
        Ok(_) => {
            println!("{} file found", filename)
        }
        Err(err) if err.kind() == ErrorKind::NotFound => {
            println!("The file does not exist. Creating file {}", filename);
            match fs::File::create(filename) {
                Ok(_) => {
                    println!("{} file is created", filename)
                }
                Err(err) => {
                    println!("Could not create file {}: {}", filename, err);
                }
            }
        }
        Err(err) => println!("Error opening file {}: {}", filename, err),
    }
}
pub fn is_file_empty(filename: &str) -> Result<bool, io::Error> {
    let file_metadata = fs::metadata(filename)?;
    Ok(file_metadata.len() == 0)
}
pub fn define_userinfo(filename: &str) -> UserInfo {
    let mut info_vec: Vec<String> = Vec::new();
    println!("Enter client_id: ");
    collect_userinfo(&mut info_vec);
    println!("Enter client_secret: ");
    collect_userinfo(&mut info_vec);
    println!("Enter username: ");
    collect_userinfo(&mut info_vec);
    println!("Enter password: ");
    collect_userinfo(&mut info_vec);
    write_to_file(filename, &info_vec);
    let userinfo = UserInfo {
        client_id: info_vec.remove(0),
        client_secret: info_vec.remove(0),
        username: info_vec.remove(0),
        password: info_vec.remove(0),
    };
    userinfo
}

fn collect_userinfo(info_vec: &mut Vec<String>) {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => info_vec.push(input.trim().to_string()),
        Err(err) => println!("Error: {}", err),
    }
}

pub fn write_to_file(filename: &str, info_vec: &Vec<String>) {
    let file = fs::File::create(filename);
    match file {
        Ok(mut file) => {
            for element in info_vec {
                match writeln!(file, "{}", element) {
                    Ok(_) => (),
                    Err(err) => println!("Write error occurred: {}", err),
                }
            }
        }
        Err(err) => println!("Could not write to file {}: {}", filename, err),
    }
}
pub fn get_userinfo(filename: &str) -> UserInfo {
    let mut info: Vec<String> = Vec::new();
    get_data(filename, &mut info);
    let userinfo = UserInfo {
        client_id: info.remove(0),
        client_secret: info.remove(0),
        username: info.remove(0),
        password: info.remove(0),
    };
    userinfo
}
pub fn get_data(filename: &str, info_vec: &mut Vec<String>) {
    let file = match fs::File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Error {}", err);
            return;
        }
    };
    let reader = io::BufReader::new(file);
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => info_vec.push(line),
            Err(err) => println!("Error reading file {}: {}", filename, err),
        }
    }
}

pub fn load_queries_from_json(file_path: &str) -> Result<HashMap<String, Vec<String>>, Error> {
    // Open the JSON file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Deserialize the JSON into the Queries struct
    let queries_data: Queries = serde_json::from_reader(reader)?;

    // Return the HashMap from the struct
    Ok(queries_data.queries)
}

pub fn copy_file(source: &str, destination: &str) {
    match fs::copy(source, destination) {
        Ok(bytes) => {
            println!("File copied successfully! {} bytes were copied.", bytes);
        }
        Err(e) => {
            eprintln!(
                "Failed to copy file from '{}' to '{}': {}",
                source, destination, e
            );
        }
    }
}
pub fn empty_file(file_path: &str) -> Result<(), io::Error> {
    match File::create(file_path) {
        Ok(_) => {
            println!("The file '{}' has been successfully emptied.", file_path);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to empty the file '{}': {}", file_path, e);
            Err(e)
        }
    }
}
pub fn del_file(file_path: &str) -> Result<(), io::Error> {
    match fs::remove_file(file_path) {
        Ok(_) => {
            println!("File '{}' has been deleted successfully.", file_path);
            Ok(())
        }
        Err(e) => {
            eprintln!("Error deleting file '{}': {}", file_path, e);
            Err(e)
        }
    }
}
