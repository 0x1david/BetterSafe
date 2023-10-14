pub use super::TerminalCommand;
use std::process::exit;

pub fn parse(args: Vec<String>, terminal_command: &mut TerminalCommand) -> &mut TerminalCommand {
    let arg = match args.get(0) {
        Some(arg) => arg,
        None => return terminal_command,
    };

    let first_char = arg
        .chars()
        .nth(0)
        .expect("Parse with no arguments should have returned.");

    if first_char == '-' {
        let second_char = match arg.chars().nth(1) {
            Some(second_char) => second_char,
            None => {
                eprintln!("No argument given following a '-'.");
                exit(1);
            }
        };

        if second_char == '-' && arg.len() > 2 {
            terminal_command.add_arg(&arg[2..].to_string());
        } else if second_char != '-' {
            arg[1..].chars().for_each(|character| {
                terminal_command.add_arg(&character.to_string())
            });
        } else {
            eprintln!("No argument given following a '--'.");
            exit(1);
        }
    } else {
        terminal_command.add_path(&arg);
    }
    parse(args[1..].to_vec(), terminal_command)
}
