use super::constants::get_archive_dir;
use std::env;
use std::fs;
use std::io;
use std::io::ErrorKind;
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

pub fn default_action(path: &String) -> () {
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
