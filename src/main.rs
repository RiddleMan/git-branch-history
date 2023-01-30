use crate::args::Args;
use crate::command::Command;
use std::error::Error;

mod args;
mod command;
mod repository;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    Command::run(&args)
}
