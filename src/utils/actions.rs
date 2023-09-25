use super::helpers::{chain_climb_directories, get_alternate_path};
use std::env;
use std::fs::{create_dir_all, rename};
use std::io;
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

pub fn default_action(source_dir: &str) -> io::Result<()> {
    println!("{} will be archived", source_dir);
    let source_dir_buf = PathBuf::from(source_dir);
    let target_dir = get_alternate_path(Some(source_dir_buf));
    match rename(source_dir, &target_dir) {
        Ok(_) => return Ok(()),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                create_dir_all(&target_dir)?;
                rename(source_dir, target_dir)?;
                return Ok(());
            }
            io::ErrorKind::AlreadyExists => {
                eprintln!("File with the same name already exists in the archive.");
                exit(1);
            }
            io::ErrorKind::PermissionDenied => {
                eprintln!("Insufficient permissions to move the file, try elevating permissions or changing where .archives is located.");
                exit(1);
            }
            _ => return Err(e),
        },
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

pub fn portal() -> io::Result<()> {
    let target_dir_str = get_alternate_path(None);
    println!("{}", target_dir_str);
    let target_dir = PathBuf::from(&target_dir_str);
    let shell = env::var("SHELL").unwrap_or_else(|_| "sh".into());
    chain_climb_directories(target_dir);
    Command::new(shell).status()?;
    Ok(())
}
