//! A port of TaskWarrior from C++
//!
//! See https://github.com/GothenburgBitFactory/taskwarrior for the original code, MIT Licensed.

use std::{ env, process };
use tw::{ Context, Result };

fn main() -> Result {
    let args: Vec<String> = env::args().collect();
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    // Lightweight version checking that doesn't require initialization or any I/O.
    if args.len() == 2 && args[1] == "--version" {
        println!("{}", VERSION);
        process::exit(0);
    }

    let mut global_context = Context::new();
    global_context.initialize(&args)?;

    Ok(())
}