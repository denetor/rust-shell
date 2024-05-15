use std::path::PathBuf;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
pub enum Action {
    /// List all commands
    List,
    /// Display a file content
    Cat {
        #[structopt()]
        target_file: PathBuf,
    },
}


#[derive(Debug, StructOpt)]
#[structopt(
    name = "rust-shell",
    about = "A command line shell written to learn rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub target_file: Option<PathBuf>,
}