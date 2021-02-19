use std::process::Command;

pub struct CreateCommand {
    pub command: String,
    pub args: Vec<String>,
}

impl CreateCommand {
    pub fn run(&self) {
        let mut comd = Command::new(&self.command);
        comd.arg(&self.args[0]);
        comd.status().expect("process failed to execute");
    }
}
