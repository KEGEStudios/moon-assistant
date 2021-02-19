mod commands;

pub use crate::commands::create_command::*;

fn main() {
    let a1 = CreateCommand {
        command: String::from("next"),
        args: vec!(String::from("--help")),
    };
    a1.run();
}
