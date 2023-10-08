

use crate::utils::archive_scheduler::Record;
use crate::utils::constants::{ArchiveDuration, FileType};

use super::archive_scheduler::ArchiveScheduler;

use super::helpers::{chain_climb_directories, get_alternate_path, handle_error, determine_source_filesystem};
use std::env;
use std::fs::{remove_dir, remove_dir_all,read_dir , remove_file, rename};
use std::path::PathBuf;
use std::process::{exit, Command};

pub fn help() {
    println!("You can use the same commands as in the rm command, refer to 'man rm'.");
    println!("");
    println!("etc");
}

pub fn version(){
    println!("Current version is '0000.1a'");
}

pub fn default_action(path: &str, scheduler: &mut ArchiveScheduler) {
    let path_buf = PathBuf::from(path);
    let (target_dir, source_dir) = determine_source_filesystem(path);
    let (duration, file_type) = if path_buf.is_file() {
        (ArchiveDuration::Medium, FileType::File)
    } else if path_buf.is_dir() {
        let mut entries = read_dir(path).expect("Path validity should have been verified already.");
        match entries.next() {
            Some(_) => (ArchiveDuration::Long, FileType::Folder),
            None => (ArchiveDuration::Short, FileType::File) //TODO: remove instantly 
        }
    } else {
        eprintln!("The path is neither a regular file nor a directory. Exiting.");
        exit(1);
    };

    println!("source:{}    target:{}", source_dir, target_dir) ;
    match rename(&source_dir, &target_dir) {
        Ok(_) => {
            scheduler.insert_record(
                    Record::new(duration, target_dir, file_type)
            )
        }
        Err(e) => handle_error(e.kind()),
    }
    let _ = scheduler.save_json();
    exit(1);
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
pub fn restore(path: &String, scheduler: &mut ArchiveScheduler) {
    let (source_dir, target_dir) = determine_source_filesystem(path);
    match rename(&source_dir, &target_dir) {
        Ok(_) => {
            scheduler.delete_record(&source_dir);
            let _ = match scheduler.save_json(){
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Could not save json with archive timers {}", e)
                }
            };
            exit(0);
        }
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
