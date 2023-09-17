pub use super::TerminalCommand;
use std::collections::HashSet;
use std::process::exit;
pub fn parse(args: Vec<String>) {
    let arg = match args.get(0) {
        Some(arg) => arg,
        None => return,
    };

    let first_char = arg.chars().nth(0).unwrap();

    if first_char == '-' {
        let second_char = match arg.chars().nth(1) {
            Some(second_char) => second_char,
            None => {
                eprintln!("No argument given following a '-'.");
                exit(1);
            }
        };

        if second_char == '-' && arg.len() > 2 {
            process_long_arg(&arg[2..]);
        } else if second_char != '-' {
            process_short_arg(&arg[1..]);
        } else {
            eprintln!("No argument given following a '--'.");
            exit(1);
        }
    } else {
        process_path(&arg);
    }
}

pub fn process_short_arg(arg: &str) -> &str {
    let allowed_flags = ['j', 'k'];
    let mut previous_flags = HashSet::new();
    for flag in arg.chars() {
        if !allowed_flags.contains(&flag) {
            eprintln!("Invalid argument(s) following a '-'.");
            exit(1);
        }
        if previous_flags.contains(&flag) {
            eprintln!("Duplicated argument(s) following a '-'.");
            exit(1);
        }
        previous_flags.insert(flag);
    }
    arg
}
pub fn process_long_arg(arg: &str) {
    unimplemented!();
}
pub fn process_path(path: &str) {
    unimplemented!();
}
