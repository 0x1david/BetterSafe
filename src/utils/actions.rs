use super::constants::{get_archive_dir, get_home_dir};
use dirs::home_dir;
use std::env;
use std::fs;
use std::io;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
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

pub fn default_action(path: &String) -> Result<(), E> {
    println!("{} will be archived", path);
    let archive_dir = get_archive_dir();
    let home_dir = home_dir()?;
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

// TODO: Implement alternate path and scale down directory if path is too deep
pub fn portal() -> io::Result<()> {
    todo!("Implement alternate path and scale down directory if path is too deep");
    let archive_dir = get_archive_dir();
    let shell = env::var("SHELL").unwrap_or_else(|_| "sh".into());
    match env::set_current_dir(&archive_dir) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                fs::create_dir(&archive_dir)?;
                env::set_current_dir(&archive_dir)?;
            }
            ErrorKind::PermissionDenied => {
                eprintln!("Insufficient permissions to portal to enter archive directory. Change target directory in '/etc/environment.d/bettersafe.conf' or elevate the command.");
                exit(1);
            }
            _ => {
                eprintln!("Portal couldn't be used due to following issue {}.", e);
                exit(1);
            }
        },
    }
    Command::new(shell).status()?;
    Ok(())
}

fn get_alternate_path(cwd_option: Option<PathBuf>) -> String {
    let cwd = match cwd_option {
        Some(path) => path,
        None => match env::current_dir() {
            Ok(cwd) => cwd,
            Err(_) => {
                eprintln!("Cannot properly access the file system on your device.");
                exit(1);
            }
        },
    };

    let archive_dir = get_archive_dir().to_string_lossy().to_string();
    let home_dir = get_home_dir().to_string_lossy().to_string();
    let cwd_str = cwd.to_string_lossy().to_string();

    if cwd.starts_with(&home_dir) {
        let target_path = cwd_str.replace(&home_dir, &archive_dir);
        return target_path;
    } else if cwd.starts_with(&archive_dir) {
        let target_path = cwd_str.replace(&archive_dir, &archive_dir);
        return target_path;
    } else {
        return archive_dir;
    }
}

// TODO: Create Directories if the path for the file creation does not exist
fn chain_create_directories(path: PathBuf) {
    unimplemented!();
}
