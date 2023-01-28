use crate::args::Args;

mod args;

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
