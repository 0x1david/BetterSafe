use super::constants::{get_archive_dir, get_home_dir};
use std::env;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::exit;

pub fn get_alternate_path(working_directory_option: Option<PathBuf>) -> String {
    let working_directory = match working_directory_option {
        Some(path) => path,
        None => match env::current_dir() {
            Ok(working_directory) => working_directory,
            Err(_) => {
                eprintln!("Cannot properly access the file system on your device.");
                exit(1);
            }
        },
    };

    let archive_dir = get_archive_dir().to_string_lossy().to_string();
    let home_dir = get_home_dir().to_string_lossy().to_string();
    let working_directory_str = working_directory.to_string_lossy().to_string();

    if working_directory.starts_with(&home_dir) {
        let target_path = working_directory_str.replace(&home_dir, &archive_dir);
        return target_path;
    } else if working_directory.starts_with(&archive_dir) {
        let target_path = working_directory_str.replace(&archive_dir, &archive_dir);
        return target_path;
    } else {
        return archive_dir;
    }
}

// TODO: Create Directories if the path for the file creation does not exist
pub fn chain_create_directories(path: PathBuf) {
    unimplemented!();
}

pub fn chain_climb_directories(target: PathBuf) {
    let mut current_try = target;
    let last_resort = get_archive_dir();
    loop {
        match env::set_current_dir(&current_try) {
            Ok(_) => {
                break;
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    if current_try == last_resort {
                        eprintln!(
                            "Failed to change directory even after ascending to {:?}",
                            &last_resort
                        );
                        exit(1);
                    }
                    if let Some(parent) = current_try.parent() {
                        current_try = parent.to_path_buf();
                    } else {
                        eprintln!("Reached root {:?}", &current_try);
                        exit(1);
                    }
                }
                ErrorKind::PermissionDenied => {
                    eprintln!("Insufficient permissions to portal to enter archive directory. Change target directory in '/etc/environment.d/bettersafe.conf' or elevate the command.");
                    exit(1);
                }
                _ => {
                    eprintln!("Portal couldn't be used due to the following issue: {}", e);
                    exit(1);
                }
            },
        }
    }
}
