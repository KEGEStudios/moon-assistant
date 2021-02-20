use std::env;

mod commands;
mod tools;

mod manager_commands;

pub use manager_commands::*;

fn main() {
    let manager: ManagerCoomands = ManagerCoomands::new();
    let args: Vec<String> = env::args().collect();
    manager.init(args).run();
}
