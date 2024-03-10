mod action;
mod command;

use clap::Parser;
use command::SubCommand;

fn main() {
    let args = command::Command::parse();

    match args.action {
        SubCommand::Dump(command) => {
            action::dump::run(command.value);
        }
        SubCommand::Clean(command) => {}
    }
}
