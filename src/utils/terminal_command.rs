use std::collections::HashMap;

use super::Arguments;

macro_rules! arg_map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $( map.insert(String::from($key), $value); )*
        map
    }};
}

pub struct TerminalCommand {
    path: String,
    arg_map: HashMap<String, Arguments>,
    args: Vec<Arguments>,
}

impl TerminalCommand {
     pub fn new() -> Self {
        let arg_map = arg_map! {
            "force" => Arguments::Force,
            "Force" => Arguments::Force,
            "f" => Arguments::Force,
            "F" => Arguments::Force,
            "interactive" => Arguments::Interactive,
            "Interactive" => Arguments::Interactive,
            "i" => Arguments::Interactive,
            "recursive" => Arguments::Recursive,
            "Recursive" => Arguments::Recursive,
            "r" => Arguments::Recursive,
            "dir" => Arguments::Directory,
            "Dir" => Arguments::Directory,
            "verbose" => Arguments::Verbose,
            "Verbose" => Arguments::Verbose,
            "help" => Arguments::Help,
            "Help" => Arguments::Help,
            "version" => Arguments::Version,
            "Version" => Arguments::Version,
            "v" => Arguments::Version,
            "trash" => Arguments::Trash,
            "Trash" => Arguments::Trash,
            "T" => Arguments::Trash,
            "abandon" => Arguments::Abandon,
            "Abandon" => Arguments::Abandon,
            "restore" => Arguments::Restore,
            "Restore" => Arguments::Restore,  
            "archive" => Arguments::Archive,
            "Archive" => Arguments::Archive,
            "a" => Arguments::Archive,
        };
        Self {
            path: String::new(),
            arg_map,
            args: Vec::new(),

        }
    }
    pub fn add_path(&mut self, path: &str) {
        self.path = path.to_string();
        println!("Path: {}", path)
    }
    pub fn add_arg(&mut self, arg: String) {
        let argument = self.arg_map[&arg];
        self.args.push(argument);
        println!("Arg: {}", argument);
    }
    pub fn execute_command(&self) {
        unimplemented!();
    }
}
