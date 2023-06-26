use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    /*
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username);
    Ok(username)
     */
    fs::read_to_string("hello.txt")
}

fn main() {
    /*
    match read_username_from_file() {
        Ok(user_name) => println!("user_name: {user_name}"),
        Err(e) => panic!("{e}")
    } */

    let username = read_username_from_file()
        .expect("Failed to read username");
    println!("username: {username}");
}
