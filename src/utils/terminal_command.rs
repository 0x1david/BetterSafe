use super::{
    actions::{default_action, help, portal, restore, trash, version},
    archive_scheduler::ArchiveScheduler,
    constants::get_home_dir_str,
};
use std::{env::current_dir, process::exit};

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
    restore: bool,
    archive: bool,
    portal: bool,
    print_json: bool,
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
            restore: false,
            archive: false,
            portal: false,
            print_json: false,
        }
    }

    pub fn add_path(&mut self, path: &str) {
        let input_path = path.to_string();
        let abs_path = if input_path.starts_with("/") {
            input_path
        } else if input_path.contains("~") {
            input_path.replace("~", &get_home_dir_str())
        } else {
            match current_dir() {
                Ok(cwd) => cwd.join(input_path).to_string_lossy().to_string(),
                Err(e) => {
                    eprintln!("Failed to get current working directory: {}", e);
                    exit(1);
                }
            }
        };
        self.path = abs_path;
    }
    pub fn add_arg(&mut self, arg: &str) {
        match arg.to_lowercase().as_str() {
            "force" => self.force = true,
            "f" => self.force = true,
            "interactive" => self.interactive = true,
            "i" => self.interactive = true,
            "recursive" => self.recursive = true,
            "r" => self.recursive = true,
            "d" => self.directory = true,
            "dir" => self.directory = true,
            "verbose" => self.verbose = true,
            "help" => self.help = true,
            "version" => self.version = true,
            "v" => self.version = true,
            "trash" => self.trash = true,
            "t" => self.trash = true,
            "restore" => self.restore = true,
            "archive" => self.archive = true,
            "a" => self.archive = true,
            "p" => self.portal = true,
            "portal" => self.portal = true,
            "debug-json" => self.print_json = true,
            _ => {
                eprintln!("Unrecognized argument '{}'", arg);
                exit(1);
            }
        }
    }
    pub fn execute(&self, archive_scheduler: &mut ArchiveScheduler) {
        if self.help {
            help()
        } else if self.print_json {
            archive_scheduler.debug_print_records()
        } else if self.version {
            version()
        } else if self.portal {
            portal();
        } else if self.trash {
            trash(&self.path, self.recursive, self.directory)
        } else if self.restore {
            restore(&self.path, archive_scheduler)
        } else {
            default_action(&self.path, archive_scheduler);
        }
    }
}
