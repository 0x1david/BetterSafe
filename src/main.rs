mod utils;

use std::env::args;
use std::process::exit;
use utils::parse;
use utils::TerminalCommand;

fn main() {
    let terminal_command = TerminalCommand::new();
    let mut args: Vec<String> = args().collect();
    if !args.is_empty() {
        args.remove(0);
    } else {
        eprintln!("No arguments provided");
        exit(1);
    }
    parse(args)
}
