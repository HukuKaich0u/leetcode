use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::Open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // username_fileをusername変数にStringとして読み出している
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    let mut username_file = File::open("hello.txt")?.
        read_to_string(&mut username)?;
    Ok(username)
}

use std::{fs, io};

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
