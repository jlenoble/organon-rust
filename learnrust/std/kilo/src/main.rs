use std::{ env, io::stdout, process };
use kilo::{ config::Config, editor::Editor };

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let stdout = stdout();
    let mut editor = Editor::new(config, stdout);

    editor.start();
}