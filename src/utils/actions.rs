use chrono::DateTime;

use crate::utils::archive_scheduler::Record;
use crate::utils::constants::{ArchiveDuration, FileType};

use super::archive_scheduler::ArchiveScheduler;
use super::constants::get_home_dir;

use super::helpers::{chain_climb_directories, get_alternate_path, handle_error};
use std::env;
use std::fs::{remove_dir, remove_dir_all,read_dir , remove_file, rename};
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

pub fn default_action(path: &str, scheduler: &mut ArchiveScheduler) {
    let path_buf = PathBuf::from(path);
    let (source_dir, target_dir) = if !path.contains(&get_home_dir().to_string_lossy().to_string())
    {
        println!("true");
        (get_alternate_path(Some(path_buf)), path.to_string())
    } else {
        println!("false");
        (path.to_string(), get_alternate_path(Some(path_buf)))
    };
    println!("target: {}, source {}", target_dir, source_dir);
    let (duration, file_type) = if path_buf.is_file() {
        (ArchiveDuration::Medium, FileType::File)
    } else if path_buf.is_dir() {
        let mut entries = read_dir(path).expect("Path vality should have been verified already.");
        match entries.next() {
            Some(_) => (ArchiveDuration::Long, FileType::Folder),
            None => (ArchiveDuration::Short, FileType::File) //TODO: remove instantly 
        }
    } else {
        eprintln!("The path is neither a regular file nor a directory. Exiting.");
        exit(1);
    };

 
    match rename(&source_dir, &target_dir) {
        Ok(_) => {
            scheduler.insert_record(
                    Record::new(duration, target_dir, file_type)
            )
        }
        Err(e) => handle_error(e.kind()),
    }
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

// TODO: remove archive note
pub fn restore(path: &String, scheduler: &mut ArchiveScheduler) -> () {
    let path_buf = PathBuf::from(path);
    let (source_dir, target_dir) = if path.contains(&get_home_dir().to_string_lossy().to_string()) {
        (get_alternate_path(Some(path_buf)), path.to_string())
    } else {
        (path.to_string(), get_alternate_path(Some(path_buf)))
    };
    match rename(&source_dir, &target_dir) {
        Ok(_) => exit(0),
        Err(e) => handle_error(e.kind()),
    }
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
