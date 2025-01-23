use super::Userinfo;
use std::{
    fs,
    io::{self, BufRead, ErrorKind, Write},
};
pub fn check_for_conf(filename: &str) {
    match fs::File::open(filename) {
        Ok(_) => {
            println!("File found")
        }
        Err(err) if err.kind() == ErrorKind::NotFound => {
            println!("The file does not exist. Creating file...");
            match fs::File::create(filename) {
                Ok(_) => {
                    println!("File is created")
                }
                Err(err) => {
                    println!("Could not create file. Error: {}", err);
                }
            }
        }
        Err(err) => println!("Error: {}", err),
    }
}
pub fn is_file_empty(filename: &str) -> Result<bool, std::io::Error> {
    let file_metadata = fs::metadata(filename)?;
    Ok(file_metadata.len() == 0)
}
pub fn define_userinfo(filename: &str) -> Userinfo {
    let mut info_vec: Vec<String> = Vec::new();
    println!("Enter client_id: ");
    store_val(&mut info_vec);
    println!("Enter client_secret: ");
    store_val(&mut info_vec);
    println!("Enter username: ");
    store_val(&mut info_vec);
    println!("Enter password: ");
    store_val(&mut info_vec);
    let userinfo = Userinfo {
        client_id: info_vec[0].clone(),
        client_secret: info_vec[1].clone(),
        username: info_vec[2].clone(),
        password: info_vec[3].clone(),
    };
    write_to_file(filename, &info_vec);
    userinfo
}

fn store_val(info_vec: &mut Vec<String>) {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => info_vec.push(input.trim().to_string()),
        Err(err) => println!("Error: {}", err),
    }
}

fn write_to_file(filename: &str, info_vec: &Vec<String>) {
    let file = fs::File::create(filename);
    match file {
        Ok(mut file) => {
            for element in info_vec {
                match writeln!(file, "{}", element) {
                    Ok(_) => (),
                    Err(err) => println!("Write error occured: {}", err)
                    }
                
            }
        }
        Err(err) => println!("Error {}", err)
    }
}
pub fn get_userinfo(filename: &str) -> Userinfo {
    let mut info_vec: Vec<String> = Vec::new();
    get_data(filename, &mut info_vec);
    let userinfo = Userinfo {
        client_id: info_vec[0].clone(),
        client_secret: info_vec[1].clone(),
        username: info_vec[2].clone(),
        password: info_vec[3].clone(),
    };
    userinfo
}
fn get_data(filename: &str, info_vec: &mut Vec<String>) {
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
            Err(err) => println!("Error reading file: {}", err)
        }
    }
}