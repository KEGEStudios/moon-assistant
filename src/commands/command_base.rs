use std::process::Command;

type Task = fn(command :CommandBase);

pub struct CommandBase {
    pub command: String,
    pub args: Vec<String>,
    pub task: Task
}

impl CommandBase {
    pub fn run(&self) {
        let mut comd = Command::new(&self.command);
        for arg in &self.args{
            comd.arg(arg);
        }
        comd.status().expect("process failed to execute");
    }
}
