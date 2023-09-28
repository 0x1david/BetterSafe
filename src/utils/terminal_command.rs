use crate::utils::constants::get_home_dir;

use super::actions::{archive, default_action, portal, trash};
use std::{fs, process::exit};

#[derive(Debug)]
pub struct TerminalCommand {
    path: String,
    force: bool,
    interactive: bool,
    recursive: bool,
    directory: bool,
    verbose: bool,
    help: bool,
    version: bool,
    trash: bool,
    abandon: bool,
    restore: bool,
    archive: bool,
    portal: bool,
}

impl TerminalCommand {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            force: false,
            interactive: false,
            recursive: false,
            directory: false,
            verbose: false,
            help: false,
            version: false,
            trash: false,
            abandon: false,
            restore: false,
            archive: false,
            portal: false,
        }
    }

    pub fn add_path(&mut self, path: &str) {
        let input_path = path.to_string();
        let abs_path = if input_path.starts_with("/") {
            input_path
        } else if input_path.contains("~") {
            input_path.replace("~", &get_home_dir().to_string_lossy().to_string())
        } else {
            match fs::canonicalize(input_path) {
                Ok(abspath) => abspath.to_string_lossy().to_string(),
                Err(e) => {
                    eprintln!("Failed to process specified path");
                    exit(1);
                }
            }
        };
        self.path = abs_path;
        println!("Path: {}", path)
    }
    pub fn add_arg(&mut self, arg: &str) {
        match arg {
            "force" => self.force = true,
            "Force" => self.force = true,
            "f" => self.force = true,
            "F" => self.force = true,
            "interactive" => self.interactive = true,
            "Interactive" => self.interactive = true,
            "i" => self.interactive = true,
            "recursive" => self.recursive = true,
            "Recursive" => self.recursive = true,
            "r" => self.recursive = true,
            "d" => self.directory = true,
            "dir" => self.directory = true,
            "Dir" => self.directory = true,
            "verbose" => self.verbose = true,
            "Verbose" => self.verbose = true,
            "help" => self.help = true,
            "Help" => self.help = true,
            "version" => self.version = true,
            "Version" => self.version = true,
            "v" => self.version = true,
            "trash" => self.trash = true,
            "Trash" => self.trash = true,
            "t" => self.trash = true,
            "T" => self.trash = true,
            "abandon" => self.abandon = true,
            "Abandon" => self.abandon = true,
            "restore" => self.restore = true,
            "Restore" => self.restore = true,
            "archive" => self.archive = true,
            "Archive" => self.archive = true,
            "a" => self.archive = true,
            "p" => self.portal = true,
            "portal" => self.portal = true,
            _ => {
                eprintln!("Unrecognized argument '{}'", arg);
                exit(1);
            }
        }
    }
    pub fn execute(&self) {
        if self.portal {
            let _ = portal();
        }
        if self.trash {
            trash(&self.path, self.recursive, self.directory)
        } else {
            default_action(&self.path);
        }
    }
}
