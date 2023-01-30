use crate::args::{Arguments, GitBranchHistoryCommand};
use crate::repository::get_list;
use std::error::Error;

pub fn display_list(count: usize) -> Result<(), Box<dyn Error>> {
    get_list(count)?.iter().enumerate().for_each(|(idx, item)| {
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
        match args.command {
            GitBranchHistoryCommand::List { length } => {
                display_list(length)?;
            }
            GitBranchHistoryCommand::PopBranch => {}
            GitBranchHistoryCommand::Checkout { .. } => {}
        };
        get_list(10)?;
        println!("{:?}", args);
        Ok(())
    }
}
