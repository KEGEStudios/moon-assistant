pub use crate::commands::command_base::*;
pub use crate::tools::stack::*;

use std::env;
use std::fs;

pub struct ManagerCoomands {
    pub stack: Stack<CommandBase>,
}

fn next_command(command: CommandBase) {
    command.run();
}

fn moon_version(command: CommandBase) {
    command.run();
    let home = env::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    assert!(env::set_current_dir(home + "/opt/moon-assistant").is_ok());

    let contents =
        fs::read_to_string("assets/version").expect("Something went wrong reading the file VERSION");
    println!("{}", contents);
}

fn moon_help(command: CommandBase) {
    //command.run();
    let home = env::home_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    assert!(env::set_current_dir(home + "/opt/moon-assistant").is_ok());

    let contents =
        fs::read_to_string("assets/help").expect("Something went wrong reading the file HELP");
    println!("{}", contents);
}

impl ManagerCoomands {
    pub fn new() -> Self {
        ManagerCoomands {
            stack: Stack::new(),
        }
    }

    pub fn init(mut self, args: Vec<String>) -> Self {
        let mut it = 0;
        for arg in args.iter() {
            match arg.as_str() {
                "-nh" => {
                    self.stack.push(CommandBase {
                        command: String::from("next"),
                        args: vec![String::from("-h")],
                        task: next_command,
                    });
                }
                "-nv" => {
                    self.stack.push(CommandBase {
                        command: String::from("next"),
                        args: vec![String::from("-v")],
                        task: next_command,
                    });
                }
                "-h" | "--help" => {
                    self.stack.push(CommandBase {
                        command: String::from(""),
                        args: vec![String::from("")],
                        task: moon_help,
                    });
                }
                "-v" | "--verson" => {
                    self.stack.push(CommandBase {
                        command: String::from("next"),
                        args: vec![String::from("-v")],
                        task: moon_version,
                    });
                }
                "-nupgrade" => {
                    self.stack.push(CommandBase {
                        command: String::from("next"),
                        args: vec![String::from("upgrade")],
                        task: next_command,
                    });
                }
                "ncreate" => {
                    self.stack.push(CommandBase {
                        command: String::from("next"),
                        args: vec![String::from("create"), String::from(args[it + 1].as_str())],
                        task: next_command,
                    });
                }
                "create" => {
                    self.stack.push(CommandBase {
                        command: String::from("next"),
                        args: vec![String::from("mcreate"), String::from(args[it + 1].as_str())],
                        task: next_command,
                    });
                }
                "build" => {
                    self.stack.push(CommandBase {
                        command: String::from("next"),
                        args: vec![String::from("build")],
                        task: next_command,
                    });
                }
                "run" => {
                    self.stack.push(CommandBase {
                        command: String::from("next"),
                        args: vec![String::from("run")],
                        task: next_command,
                    });
                }
                _ => println!(""),
            }
            it += 1;
        }
        return self;
    }

    pub fn run(mut self) {
        self.stack.rev();
        while self.stack.length() > 0 {
            let command: Option<CommandBase> = self.stack.pop();
            match command {
                Some(value) => {
                    let run_task = value.task;
                    run_task(value);
                }
                None => println!("has no value"),
            }
        }
    }
}
