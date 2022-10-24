use crossterm::{ cursor, QueueableCommand, Result, terminal };
use std::io::{ Stdout, Write };
use crate::error_handler::ErrorHandler;

pub struct Screen {
    columns: u16,
    rows: u16,
    stdout: Stdout,
}

impl Screen {
    pub fn new(stdout: Stdout, error_handler: ErrorHandler) -> Self {
        let mut screen = Screen { columns: 1, rows: 1, stdout };

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
                Screen { columns, rows, stdout: screen.stdout }
            }
            Err(e) => {
                let ref mut screen = screen;
                error_handler.die(screen, format!("could not determine terminal size: {:?}", e));
            }
        }
    }

    fn draw_rows(&mut self) -> Result<&mut Self> {
        let rows = self.rows - 1;

        for _ in 0..rows {
            write!(self.stdout, "~\r\n")?;
        }

        write!(self.stdout, "~")?;
        self.stdout.flush()?;

        Ok(self)
    }

    pub fn refresh(&mut self) -> Result<&mut Self> {
        self.clear()?.draw_rows()?.move_cursor(0, 0)
    }

    pub fn clear(&mut self) -> Result<&mut Self> {
        self.stdout
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .flush()?;
        Ok(self)
    }

    fn move_cursor(&mut self, col: u16, row: u16) -> Result<&mut Self> {
        self.stdout.queue(cursor::MoveTo(col, row))?;
        Ok(self)
    }
}