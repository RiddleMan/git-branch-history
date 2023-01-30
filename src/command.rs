use crate::args::Arguments;
use crate::repository::get_list;
use std::error::Error;

pub struct Command;

impl Command {
    pub fn run(args: &Arguments) -> Result<(), Box<dyn Error>> {
        get_list(10)?;
        println!("{:?}", args);
        Ok(())
    }
}
