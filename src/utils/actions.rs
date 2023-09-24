use super::constants::get_archive_dir;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::exit;
use std::process::Command;

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
    let archive_dir = get_archive_dir();
    unimplemented!()
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

pub fn portal() -> io::Result<()> {
    let archive_dir = get_archive_dir();
    let shell = env::var("SHELL").unwrap_or_else(|_| "sh".into());
    env::set_current_dir(&archive_dir)?;
    Command::new(shell).status()?;
    Ok(())
}
