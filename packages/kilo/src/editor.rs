use crossterm::{ event::{ Event::Key, KeyCode, KeyEvent, KeyModifiers, poll, read } };
use std::{ io::Stdout, time::Duration };
use crate::{ error_handler::ErrorHandler, screen::Screen };

pub struct Editor {
    screen: Screen,
    error_handler: ErrorHandler,
}

impl Editor {
    pub fn new(stdout: Stdout) -> Self {
        let error_handler = ErrorHandler::new();
        Self {
            screen: Screen::new(stdout, error_handler),
            error_handler,
        }
    }

    pub fn start(&mut self) -> ! {
        loop {
            match poll(Duration::from_millis(100)) {
                Ok(true) => {
                    let _ = self.screen.refresh();
                    self.process_keypress();
                }
                Ok(false) => {}
                Err(e) => {
                    let ref mut screen = self.screen;
                    self.error_handler.die(screen, format!("failed to poll terminal: {:?}", e));
                }
            }
        }
    }

    pub fn process_keypress(&mut self) {
        let key_event = self.read_key();

        match key_event {
            KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, .. } => {
                let ref mut screen = self.screen;
                self.error_handler.clear_and_exit(screen);
            }
            KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::NONE, .. } => {
                println!("{}", c);
            }
            KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::SHIFT, .. } => {
                println!("{}", c);
            }
            _ => {
                let ref mut screen = self.screen;
                self.error_handler.die(screen, format!("unhandled key event: {key_event:?}"));
            }
        }
    }

    fn read_key(&mut self) -> KeyEvent {
        if let Ok(Key(event)) = read() {
            event
        } else {
            let ref mut screen = self.screen;
            self.error_handler.die(screen, "failed to read terminal");
        }
    }
}