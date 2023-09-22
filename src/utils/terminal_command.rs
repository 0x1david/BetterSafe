use super::Arguments;
use super::get_flag_map;

pub struct TerminalCommand {
    path: String,
    args: Vec<Arguments>,
}

impl TerminalCommand {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            args: Vec::new(),
        }
    }
    pub fn add_path(&mut self, path: &str) {
        self.path = path.to_string();
        println!("Path: {}", path)
    }
    pub fn add_arg(&mut self, arg: Arguments) {
        self.args.push(arg);
        println!("Arg: {}", arg);
    }
    pub fn execute_command(&self) {
        unimplemented!();
    }
}
