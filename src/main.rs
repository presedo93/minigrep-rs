use std::env;
use colored::*;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {}", err.bold().red());
        process::exit(1);
    });

    println!("Searching for {}", config.query.bold().green());
    println!("In file {}", config.filename.bold().green());

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
