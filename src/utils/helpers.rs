use super::constants::{get_archive_dir, get_home_dir_str};
use std::env;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::exit;

/// Returns an alternate path for a given working directory.
/// If a working directory is provided, it returns the path as is.
/// If no working directory is provided, it retrieves the current working directory.
/// If the working directory starts with the home directory, it replaces it with the archive directory.
/// If the working directory starts with the archive directory, it returns the working directory as is.
/// Otherwise, it returns the archive directory.
pub fn get_alternate_path<T: AsRef<Path>>(working_dir_option: Option<T>) -> String {

    let working_directory = working_dir_option
        .map(|path| path.as_ref().to_path_buf())
        .unwrap_or_else(|| {
            env::current_dir().unwrap_or_else(|_| {
                eprintln!("Cannot properly access the file system on your device.");
                exit(1);
            })
        });

    let archive_dir = get_archive_dir();
    let home_dir = get_home_dir_str();
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

/// Attempts to set the current directory to the specified target directory.
/// If the directory change fails due to the directory not being found,
/// it ascends the directory tree until it either succeeds or reaches the archive source directory.
/// If it reaches the archive source directory and still fails, it exits the program.
/// If the directory change fails due to permission issues, it prints an error message and exits the program.
/// For other errors, it prints a generic error message and exits the program.
pub fn chain_climb_directories<T: AsRef<Path>>(target: T) {
    let mut current_directory = target.as_ref().to_path_buf();
    let archive_source = get_archive_dir();
    loop {
        match env::set_current_dir(&current_directory) {
            Ok(_) => {
                break;
            }
            Err(ref e) => match e.kind() {
                ErrorKind::NotFound => {
                    if current_directory.to_string_lossy().to_string() == archive_source {
                        eprintln!(
                            "Failed to change directory even after ascending to {:?}",
                            &archive_source
                        );
                        exit(1);
                    }
                    if let Some(parent) = current_directory.parent() {
                        current_directory = parent.to_path_buf();
                    } else {
                        eprintln!("Reached root {:?}", &current_directory);
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

/// Handles common IO errors by printing an appropriate error message to stderr.
/// Exits the program with a non-zero exit code for all handled errors.
pub fn handle_error(ref e: ErrorKind) {
    match e {
        ErrorKind::NotFound => eprintln!("Error: The specified path does not exist."),
        ErrorKind::PermissionDenied => eprintln!("Error: Permission denied."),
        ErrorKind::InvalidInput => eprintln!("Error: The specified path is invalid."),
        ErrorKind::AlreadyExists => eprintln!("Error: The file or directory already exists."),
        _ => eprintln!("An unexpected error occurred: {:?}", e),
    }
    exit(1);
}

/// Determines whether given directory is in home directory or in archives directory
/// If the path contains the home directory, it returns a tuple with the path and its alternate.
/// Otherwise, it returns a tuple with its alternate and the path.
pub fn determine_source_filesystem(path: &str) -> (String, String) {
    let home_dir = get_home_dir_str();
    let path_buf = PathBuf::from(path);
    let alternate = get_alternate_path(Some(path_buf));

    if path.contains(&home_dir) {
        (path.to_string(), alternate)
    } else {
        (alternate, path.to_string())
    }
}
