pub enum Options {
    Force,
    Interactive,
    Recursive,
    Dir,
    Verbose,
    Version,
    Help,
    Trash,
    Abandon,
    Restore,
    Archive
}

pub fn help() -> () {
    println!("You can use the same commands as in the rm command, refer to 'man rm'.");
    println!("")
    println!("etc")
}

pub fn version() -> () {
    println!("Current version is '0000.1a'")
}

