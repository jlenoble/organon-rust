//! A port of TaskWarrior from C++
//!
//! See https://github.com/GothenburgBitFactory/taskwarrior for the original code, MIT Licensed.

use std::{ env, process };

fn main() {
    let args: Vec<String> = env::args().collect();
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    // Lightweight version checking that doesn't require initialization or any I/O.
    if args.len() == 2 && args[1] == "--version" {
        println!("{}", VERSION);
        process::exit(0);
    }
}