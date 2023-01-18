use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // Turns iterator into Vector
    let config = Config::new(&args).unwrap_or_else(|err| {
        //NOTE: `eprintln!` outputs to stderror. `println!` to stdoutput
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}