use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    match really_short_read_username_from_file() {
        Ok(username) => println!("found username: {}", username),
        Err(message) => println!("Could not get username from file: {}", message),
    };
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    // read_to_string advances the file pointer which is a mutation, so mut is needed.
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn short_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)
}

fn shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn really_short_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
