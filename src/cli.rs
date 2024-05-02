use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
pub enum Action {
    /// List all commands
    List,
}


#[derive(Debug, StructOpt)]
#[structopt(
    name = "rust-shell",
    about = "A command line shell written to learn rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}