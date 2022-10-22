use std::env;
use std::process;

use rust_bbdc::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem when parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = rust_bbdc::run(config) {
        eprintln!("Problem when running: {err}");
        process::exit(1);
    }
}
