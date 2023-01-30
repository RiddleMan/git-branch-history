use crate::repository::Repository;
use std::error::Error;

mod repository;

fn main() -> Result<(), Box<dyn Error>> {
    let repo = Repository::new()?;

    repo.checkout_last()
}
