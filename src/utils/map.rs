use maplit::hashmap;
use super::Arguments;
use std::collections::HashMap;

pub fn get_flag_map() -> HashMap<&'static str, super::Arguments> {
    hashmap! {
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
    }
}
