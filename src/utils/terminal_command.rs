pub struct TerminalCommand {
    path: String,
    args: Vec<char>,
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
    }
    pub fn add_arg(&mut self, arg: char) {
        self.args.push(arg);
    }
    pub fn execute_command(&self) {
        unimplemented!();
    }
}
