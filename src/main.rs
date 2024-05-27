use structopt::StructOpt;
mod cli;
mod cat_command;

use cli::{CommandLineArgs};
use cat_command::CatCommand;

fn main() {
    // println!("{:#?}", cli::CommandLineArgs::from_args());

    // Get the command-line arguments.
    let CommandLineArgs {
        action,
    } = CommandLineArgs::from_args();

    match action {
        _cat => CatCommand::do_cat(),
    }
}
