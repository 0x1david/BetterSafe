use super::constants::ARCHIVE_DIR;
use std::fs;
use std::process::exit;

pub fn help() -> () {
    println!("You can use the same commands as in the rm command, refer to 'man rm'.");
    println!("");
    println!("etc");
}

pub fn version() -> () {
    println!("Current version is '0000.1a'");
}

pub fn remove(path: &String) -> () {
    println!("{} will be archived", path);
    match fs::rename(path, ARCHIVE_DIR) {
        Ok(()) => println!("{} has been archived", path),
        Err(e) => {
            eprintln!("Archiving Failed {}", e);
            exit(1);
        }
    }
}

pub fn archive(path: &String) -> () {
    unimplemented!();
}

pub fn trash(path: &String) -> () {
    unimplemented!();
}

pub fn restore(path: &String) -> () {
    unimplemented!();
}

pub fn abandon(path: &String) -> () {
    unimplemented!();
}

pub fn portal(path: &String) -> () {
    unimplemented!();
}
