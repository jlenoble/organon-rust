use std::{ time::Duration, io::stdout };
use crossterm::event::poll;
use kilo::editor::Editor;

fn main() {
    let stdout = stdout();
    let editor = Editor::new(stdout);

    loop {
        match poll(Duration::from_millis(100)) {
            Ok(true) => {
                let _ = editor.refresh_screen();
                editor.process_keypress();
            }
            Ok(false) => {}
            Err(e) => {
                editor.die(format!("failed to poll terminal: {:?}", e));
            }
        }
    }
}