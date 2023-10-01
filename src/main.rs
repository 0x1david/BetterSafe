mod utils;

use std::env::args;
use std::process::exit;
use utils::parse;
use utils::TerminalCommand;
use crate::utils::archive_scheduler::ArchiveScheduler;

fn main() {
    let mut unparsed_command = TerminalCommand::new();
    let mut args: Vec<String> = args().collect();
    let mut archive_scheduler = ArchiveScheduler::new().expect("Failed connecting to the archive file.");
    if !args.is_empty() {
        args.remove(0);
    } else {
        eprintln!("No arguments provided");
        exit(1);
    }

    // TODO: Async Archive Scheduler
    archive_scheduler.handle_due_records();

    let terminal_command = parse(args, &mut unparsed_command);
    println!("{:?}", terminal_command);
    terminal_command.execute(archive_scheduler);
}
