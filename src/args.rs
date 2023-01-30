use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum GitBranchHistoryCommand {
    /// List history of branch checkouts
    List {
        /// Max history length
        #[arg(short, long, default_value_t = 10)]
        length: usize,
    },
    /// Navigate to previous branch from history
    PopBranch,
    /// Checkout a branch from history
    Checkout {
        /// No of branch to select from history
        no: Option<usize>,
    },
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
