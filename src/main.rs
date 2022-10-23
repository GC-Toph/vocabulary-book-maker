use std::env;
use std::process;

use rust_bbdc::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem when parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = rust_bbdc::run(config) {
        eprintln!("Problem when running: {err}");
        process::exit(1);
    }
}
