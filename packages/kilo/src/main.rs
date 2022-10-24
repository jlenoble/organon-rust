use std::io::stdout;
use kilo::editor::Editor;

fn main() {
    let stdout = stdout();
    let mut editor = Editor::new(stdout);

    editor.start();
}