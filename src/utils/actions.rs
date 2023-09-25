use super::helpers::{chain_climb_directories, get_alternate_path};
use std::env;
use std::io;
use std::path::PathBuf;
use std::process::Command;

pub fn help() -> () {
    println!("You can use the same commands as in the rm command, refer to 'man rm'.");
    println!("");
    println!("etc");
}

pub fn version() -> () {
    println!("Current version is '0000.1a'");
}

pub fn default_action(path: &String) -> () {
    // println!("{} will be archived", path);
    // let archive_dir = get_archive_dir();
    // let home_dir = home_dir()?;
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
    let target_dir_str = get_alternate_path(None);
    println!("{}", target_dir_str);
    let target_dir = PathBuf::from(&target_dir_str);
    let shell = env::var("SHELL").unwrap_or_else(|_| "sh".into());
    chain_climb_directories(target_dir);
    Command::new(shell).status()?;
    Ok(())
}
