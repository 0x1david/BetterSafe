use core::fmt;
use std::marker::Copy;

#[derive(Copy, Clone)]
pub enum Arguments {
    Force,
    Interactive,
    Recursive,
    Directory,
    Verbose,
    Version,
    Help,
    Trash,
    Abandon,
    Restore,
    Archive
}

impl fmt::Display for Arguments {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let descriptor = match *self {
            Arguments::Force => "Force",
            Arguments::Interactive => "Interactive",
            Arguments::Recursive => "Recursive",
            Arguments::Directory => "Directory",
            Arguments::Verbose => "Verbose",
            Arguments::Version => "Version",
            Arguments::Help => "Help",
            Arguments::Trash => "Trash",
            Arguments::Abandon => "Abandon",
            Arguments::Restore => "Restore",
            Arguments::Archive => "Archive",
        };
        
        write!(f, "{}", descriptor)
    }
}

