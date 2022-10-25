use crossterm::{ event::{ Event::Key, KeyCode, KeyEvent, KeyModifiers, poll, read }, Result };
use std::{ fs, io::Stdout, time::Duration };
use crate::{
    cursor::{ Cursor, Offset },
    error_handler::ErrorHandler,
    screen::Screen,
    config::Config,
};

pub struct Editor {
    screen: Screen,
    config: Config,
    error_handler: ErrorHandler,
    lines: Vec<String>,
    line_offset: usize,
}

impl Editor {
    pub fn new(config: Config, stdout: Stdout) -> Self {
        let error_handler = ErrorHandler::new();

        Self {
            screen: Screen::new(stdout, error_handler),
            config,
            error_handler,
            lines: Vec::new(),
            line_offset: 0,
        }
    }

    pub fn start(&mut self) -> ! {
        if let Err(e) = self.open() {
            self.error_handler.die(&mut self.screen, format!("failed to open file: {}", e));
        }

        let _ = self.screen.refresh(&self.lines, self.line_offset);
        loop {
            match poll(Duration::from_millis(100)) {
                Ok(true) => {
                    match self.process_keypress() {
                        Ok(Offset::Row(row)) => {
                            if row < 0 {
                                let row = -row as usize;
                                if self.line_offset > row {
                                    self.line_offset -= row;
                                } else {
                                    self.line_offset = 0;
                                }
                            } else {
                                let row = row as usize;
                                self.line_offset += row;
                                if self.line_offset > self.lines.len() {
                                    self.line_offset = self.lines.len();
                                }
                            }
                        }
                        _ => {}
                    }
                    let _ = self.screen.refresh(&self.lines, self.line_offset);
                }
                Ok(false) => {}
                Err(e) => {
                    let ref mut screen = self.screen;
                    self.error_handler.die(screen, format!("failed to poll terminal: {:?}", e));
                }
            }
        }
    }

    pub fn open(&mut self) -> Result<()> {
        let contents = fs::read_to_string(&self.config.file_path)?;

        for line in contents.lines() {
            self.lines.push(line.to_owned());
        }

        Ok(())
    }

    pub fn process_keypress(&mut self) -> Result<Offset> {
        let key_event = self.read_key();

        match key_event {
            KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, .. } => {
                let ref mut screen = self.screen;
                self.error_handler.clear_and_exit(screen);
            }

            KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::NONE, .. } => {
                return Cursor::move_up(&mut self.screen);
            }
            KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::NONE, .. } => {
                return Cursor::move_down(&mut self.screen);
            }
            KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::NONE, .. } => {
                return Cursor::move_left(&mut self.screen);
            }
            KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::NONE, .. } => {
                return Cursor::move_right(&mut self.screen);
            }

            KeyEvent { code: KeyCode::PageUp, modifiers: KeyModifiers::NONE, .. } => {
                let _ = Cursor::move_top(&mut self.screen);
            }
            KeyEvent { code: KeyCode::PageDown, modifiers: KeyModifiers::NONE, .. } => {
                let _ = Cursor::move_bottom(&mut self.screen);
            }
            KeyEvent { code: KeyCode::Home, modifiers: KeyModifiers::NONE, .. } => {
                let _ = Cursor::move_left_border(&mut self.screen);
            }
            KeyEvent { code: KeyCode::End, modifiers: KeyModifiers::NONE, .. } => {
                let _ = Cursor::move_right_border(&mut self.screen);
            }

            KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::NONE, .. } => {
                let _ = self.screen.write(c);
            }
            KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::SHIFT, .. } => {
                let _ = self.screen.write(c);
            }
            _ => {
                let ref mut screen = self.screen;
                self.error_handler.die(screen, format!("unhandled key event: {key_event:?}"));
            }
        }

        Ok(Offset::None)
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