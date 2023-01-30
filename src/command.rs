use crate::args::{Arguments, GitBranchHistoryCommand};
use crate::repository::Repository;
use std::error::Error;

pub fn display_list(repo: &Repository, count: usize) -> Result<(), Box<dyn Error>> {
    repo.get_list(count)?
        .iter()
        .enumerate()
        .for_each(|(idx, item)| {
            println!(
                "{}: {}{}",
                idx,
                item.branch,
                if !item.exists { " - deleted" } else { "" }
            )
        });

    Ok(())
}

pub struct Command;

impl Command {
    pub fn run(args: &Arguments) -> Result<(), Box<dyn Error>> {
        let repo = Repository::new()?;
        match args.command {
            GitBranchHistoryCommand::List { length } => {
                display_list(&repo, length)?;
            }
            GitBranchHistoryCommand::PopBranch => repo.checkout_last()?,
            GitBranchHistoryCommand::Checkout { .. } => {}
        };
        Ok(())
    }
}
