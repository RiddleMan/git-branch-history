use core::panicking::panic;
use git2::build::CheckoutBuilder;
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

pub struct CheckoutEntry {
    pub branch: String,
    pub exists: bool,
}

pub fn get_list(no: usize) -> Result<Vec<CheckoutEntry>, Box<dyn Error>> {
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
            let branch_name = if let Some(msg) = log.message() {
                get_from_branch_from_ref_msg(msg)
            } else {
                "".to_owned()
            };

            let exists = repo.revparse(&branch_name).map_or_else(|_| false, |_| true);

            CheckoutEntry {
                branch: branch_name,
                exists,
            }
        })
        .take(no)
        .collect();

    Ok(checkout_logs)
}

pub fn checkout_last() -> Result<(), Box<dyn Error>> {
    let last_checkout = get_list(1)?
        .first()
        .expect("There's no entry of last checkout. Is it a fresh repo?");

    if !last_checkout.exists {
        panic!("Branch of name: {} no longer exists. Run: `git branch-history checkout` to select other branch.", last_checkout.branch);
    }

    let repo = get_repo()?;
    let rev = repo.revparse(&last_checkout.branch)?.to().unwrap();

    repo.checkout_tree(rev, None)?;

    Ok(())
}
