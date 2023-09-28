use crate::utils::helpers;

use super::helpers::{chain_climb_directories, get_alternate_path, handle_error};
use std::env;
use std::fs::{create_dir_all, remove_dir, remove_dir_all, remove_file, rename};
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;
use std::process::{exit, Command};

pub fn help() -> () {
    println!("You can use the same commands as in the rm command, refer to 'man rm'.");
    println!("");
    println!("etc");
}

pub fn version() -> () {
    println!("Current version is '0000.1a'");
}

pub fn default_action(path: &str) {
    println!("{} will be archived", path);
    let path_buf = PathBuf::from(path);
    let target_dir = get_alternate_path(Some(path_buf));
    match rename(path, &target_dir) {
        Ok(_) => exit(0),
        Err(e) => handle_error(e.kind()),
    }
}

pub fn archive(path: &String) -> () {
    unimplemented!();
}

pub fn trash(path: &str, recursively: bool, directory: bool) {
    if recursively && directory {
        match remove_dir_all(&path) {
            Ok(_) => {
                exit(0);
            }
            Err(e) => handle_error(e.kind()),
        }
    } else if directory {
        match remove_dir(&path) {
            Ok(_) => {
                exit(0);
            }
            Err(e) => handle_error(e.kind()),
        }
    } else {
        match remove_file(&path) {
            Ok(_) => {
                exit(0);
            }
            Err(e) => handle_error(e.kind()),
        }
    }
}

pub fn restore(path: &String) -> () {
    unimplemented!();
}

pub fn abandon(path: &String) -> () {
    unimplemented!();
}

pub fn portal() {
    let target_dir_str = get_alternate_path(None);
    println!("{}", target_dir_str);
    let target_dir = PathBuf::from(&target_dir_str);
    let shell = env::var("SHELL").unwrap_or_else(|_| "sh".into());
    chain_climb_directories(target_dir);
    match Command::new(shell).status() {
        Ok(_) => exit(0),
        Err(e) => handle_error(e.kind()),
    }
}
