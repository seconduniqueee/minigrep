use std::{process, env};
use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(args).unwrap_or_else(|err| {
        println!("{}", format!("\nERR - problem parsing arguments: {}\n", err));
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("{}", e);
        process::exit(1);
    }
}
