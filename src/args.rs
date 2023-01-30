use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum GitBranchHistoryCommand {
    /// List history of branch checkouts
    List,
    /// Navigate to previous branch from history
    PopBranch,
    /// Checkout a branch from history
    Checkout { no: usize },
}
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Arguments {
    /// Command
    #[clap(subcommand)]
    pub command: GitBranchHistoryCommand,
}

pub struct Args;

impl Args {
    pub fn parse() -> Arguments {
        Arguments::parse()
    }
}
