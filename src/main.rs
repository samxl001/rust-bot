mod file_operations;

struct Userinfo {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
}

fn main() {
    let user_file = String::from("info.txt");
    file_operations::check_for_conf(user_file);
}
