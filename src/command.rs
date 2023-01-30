use crate::args::{Arguments, GitBranchHistoryCommand};
use crate::repository::Repository;
use std::error::Error;
use std::io;

fn display_list(repo: &Repository, count: usize) -> Result<(), Box<dyn Error>> {
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

fn checkout_interactive(repo: &Repository, length: usize) -> Result<(), Box<dyn Error>> {
    display_list(repo, length)?;

    print!("Which branch to checkout: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let no = input
        .trim()
        .parse::<usize>()
        .expect("Input is an incorrect positive number.");

    if no > length {
        panic!("Number is out of history range.");
    }

    repo.checkout_nth(no)
}

pub struct Command;

impl Command {
    pub fn run(args: &Arguments) -> Result<(), Box<dyn Error>> {
        let repo = Repository::new()?;
        match args.command {
            GitBranchHistoryCommand::List { length } => display_list(&repo, length),
            GitBranchHistoryCommand::PopBranch => repo.checkout_last(),
            GitBranchHistoryCommand::Checkout { no, length } => {
                if let Some(no) = no {
                    repo.checkout_nth(no)
                } else {
                    checkout_interactive(&repo, length)
                }
            }
        }
    }
}
