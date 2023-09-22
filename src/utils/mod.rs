pub mod map;
pub mod parser;
pub mod terminal_command;
pub mod actions;

pub use map::get_flag_map;
pub use parser::parse;
pub use terminal_command::TerminalCommand;
pub use actions::Arguments;
