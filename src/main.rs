use structopt::StructOpt;
mod cli;
use cli::{Action::*, CommandLineArgs};

fn main() {
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
