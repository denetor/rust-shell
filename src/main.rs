use structopt::StructOpt;
mod cli;
mod catCommand;

use cli::{Action::*, CommandLineArgs};
use catCommand::CatCommand;

fn main() {
    // println!("{:#?}", cli::CommandLineArgs::from_args());

    // Get the command-line arguments.
    let CommandLineArgs {
        action,
        target_file,
    } = CommandLineArgs::from_args();

    let target_file = target_file.expect("Missing target file");

    match action {
        cat => CatCommand::do_cat(target_file),
    }
}
