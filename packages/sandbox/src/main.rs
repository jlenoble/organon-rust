use std::process;

use sandbox::run;

fn main() {
    if let Err(_) = run() {
        eprintln!("Unexpected application error");
        process::exit(1);
    }
}