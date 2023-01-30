use crate::args::Args;
use crate::command::Command;

mod args;
mod command;
mod repository;

fn main() {
    let args = Args::parse();
    Command::run(&args).expect("asdf");
}
