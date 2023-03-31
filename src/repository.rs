use git2::Repository as GRepository;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;

pub struct CheckoutEntry {
    pub branch: String,
    pub exists: bool,
}

fn get_from_branch_from_ref_msg(value: &str) -> String {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^checkout: moving from (?P<from_branch>[^ ]*)").unwrap();
    }

    let caps = RE.captures(value).unwrap();

    caps["from_branch"].to_owned()
}

pub struct Repository {
    inner: GRepository,
}

impl Repository {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Repository {
            inner: GRepository::discover(std::env::current_dir()?)?,
        })
    }

    pub fn get_list(&self, no: usize) -> Result<Vec<CheckoutEntry>, Box<dyn Error>> {
        let reflog = self.inner.reflog("HEAD")?;

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

                let exists = self
                    .inner
                    .revparse(&branch_name)
                    .map_or_else(|_| false, |_| true);

                CheckoutEntry {
                    branch: branch_name,
                    exists,
                }
            })
            .take(no)
            .collect();

        Ok(checkout_logs)
    }

    #[allow(dead_code)]
    pub fn checkout_nth(&self, n: usize) -> Result<(), Box<dyn Error>> {
        let branch_list = self.get_list(n + 1)?;
        let branch = branch_list.get(n).expect("Entry doesn't exist.");

        self.checkout_branch(&branch.branch)
    }

    fn checkout_branch(&self, branch_name: &str) -> Result<(), Box<dyn Error>> {
        let revspec = self.inner.revparse_single(branch_name)?;

        self.inner.checkout_tree(&revspec, None)?;
        self.inner.set_head(&format!("refs/heads/{branch_name}"))?;

        Ok(())
    }

    pub fn checkout_last(&self) -> Result<(), Box<dyn Error>> {
        let branch_list = self.get_list(1)?;
        let last_checkout = branch_list
            .first()
            .expect("There's no entry of last checkout. Is it a fresh repo?");

        if !last_checkout.exists {
            panic!("Branch of name: {} no longer exists. Run: `git branch-history checkout` to select other branch.", last_checkout.branch);
        }

        self.checkout_branch(&last_checkout.branch)
    }
}
