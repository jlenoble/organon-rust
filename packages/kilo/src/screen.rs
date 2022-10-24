use crossterm::{ cursor, QueueableCommand, Result, style::Print, terminal };
use std::io::{ Stdout, Write };
use crate::error_handler::ErrorHandler;

pub struct Screen {
    columns: u16,
    rows: u16,
    stdout: Stdout,
    buffer: String,
}

impl Screen {
    pub fn new(stdout: Stdout, error_handler: ErrorHandler) -> Self {
        let mut screen = Screen { columns: 1, rows: 1, stdout, buffer: String::new() };

        if let Err(e) = terminal::enable_raw_mode() {
            let ref mut screen = screen;
            error_handler.die(screen, format!("failed to enable terminal raw mode: {:?}", e));
        }

        if let Err(e) = screen.clear() {
            // don't use die here in order to prevent infinite recursive calls
            // as the terminal is cleared on exit.
            eprintln!("failed to clear terminal: {:?}", e);
            error_handler.exit(1);
        }

        match terminal::size() {
            Ok((columns, rows)) => {
                let screen = screen;
                Screen { columns, rows, stdout: screen.stdout, buffer: screen.buffer }
            }
            Err(e) => {
                let ref mut screen = screen;
                error_handler.die(screen, format!("could not determine terminal size: {:?}", e));
            }
        }
    }

    fn draw_rows(&mut self) -> Result<&mut Self> {
        let last_row = self.rows - 1;
        let welcome_row = last_row / 3;

        for row in 0..last_row {
            self.move_cursor(0, row)?;

            if row < welcome_row {
                self.buffer.push_str(
                    format!("{}\r\n", terminal::Clear(terminal::ClearType::UntilNewLine)).as_str()
                );
            } else if row == welcome_row {
                self.buffer.push_str(
                    format!(
                        "{}{}\r\n",
                        "Kilo editor -- version %s",
                        terminal::Clear(terminal::ClearType::UntilNewLine)
                    ).as_str()
                );
            } else {
                self.buffer.push_str(
                    format!("~{}\r\n", terminal::Clear(terminal::ClearType::UntilNewLine)).as_str()
                );
            }
        }

        self.move_cursor(0, last_row)?;
        self.buffer.push_str(
            format!("~{}", terminal::Clear(terminal::ClearType::FromCursorDown)).as_str()
        );

        Ok(self)
    }

    fn flush(&mut self) -> Result<&mut Self> {
        self.stdout.queue(Print(self.buffer.as_str()))?.flush()?;
        self.buffer.clear();
        Ok(self)
    }

    pub fn refresh(&mut self) -> Result<&mut Self> {
        self.draw_rows()?.move_cursor(0, 0)?.flush()
    }

    pub fn clear(&mut self) -> Result<&mut Self> {
        self.buffer.push_str(
            format!(
                "{}{}",
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, 0)
            ).as_str()
        );
        self.flush()
    }

    fn cursor_position(&self) -> Result<(u16, u16)> {
        cursor::position()
    }

    fn move_cursor(&mut self, col: u16, row: u16) -> Result<&mut Self> {
        self.buffer.push_str(format!("{}", cursor::MoveTo(col, row)).as_str());
        Ok(self)
    }
}