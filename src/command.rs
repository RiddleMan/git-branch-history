use crate::args::Arguments;
use git2::Repository;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;

fn get_repo() -> Result<Repository, Box<dyn Error>> {
    Ok(Repository::discover(std::env::current_dir()?)?)
}

fn get_from_branch_from_ref_msg(value: &str) -> String {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^checkout: moving from (?P<from_branch>[^ ]*)").unwrap();
    }

    let caps = RE.captures(value).unwrap();

    caps["from_branch"].to_owned()
}

fn get_list(no: usize) -> Result<(), Box<dyn Error>> {
    let repo = get_repo()?;

    let reflog = repo.reflog("HEAD")?;

    let checkout_logs: Vec<_> = reflog
        .iter()
        .filter(|log| {
            if let Some(msg) = log.message() {
                msg.starts_with("checkout: ")
            } else {
                false
            }
        })
        .map(|log| {
            if let Some(msg) = log.message() {
                get_from_branch_from_ref_msg(msg)
            } else {
                "".to_owned()
            }
        })
        .take(no)
        .collect();

    println!("{:?}", checkout_logs);

    Ok(())
}

pub struct Command;

impl Command {
    pub fn run(args: &Arguments) -> Result<(), Box<dyn Error>> {
        get_list(1)?;
        println!("{:?}", args);
        Ok(())
    }
}
