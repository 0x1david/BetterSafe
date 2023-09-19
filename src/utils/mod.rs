pub mod parser;
pub mod terminal_command;
pub mod map;

pub use parser::parse;
pub use terminal_command::TerminalCommand;
pub use map::get_flag_map;
