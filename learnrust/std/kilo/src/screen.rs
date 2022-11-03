use crossterm::{ cursor, QueueableCommand, Result, style::Print, terminal };
use std::io::{ Stdout, Write };
use pad::{ PadStr, Alignment };
use crate::{ error_handler::ErrorHandler, cursor::Cursor };

pub struct Screen {
    columns: u16,
    rows: u16,
    stdout: Stdout,
    buffer: String,
}

impl Screen {
    pub fn new(stdout: Stdout, error_handler: ErrorHandler) -> Self {
        let mut screen = Screen {
            columns: 1,
            rows: 1,
            stdout,
            buffer: String::new(),
        };

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
                Screen { columns, rows, ..screen }
            }
            Err(e) => {
                let ref mut screen = screen;
                error_handler.die(screen, format!("could not determine terminal size: {:?}", e));
            }
        }
    }

    pub fn min_row(&self) -> u16 {
        0
    }

    pub fn max_row(&self) -> u16 {
        self.rows - 1
    }

    pub fn min_col(&self) -> u16 {
        0
    }

    pub fn max_col(&self) -> u16 {
        self.columns - 1
    }

    fn draw_rows(&mut self, lines: &Vec<String>, line_offset: usize) -> Result<&mut Self> {
        let last_row = self.rows - 1;
        let welcome_row = last_row / 3;
        let mut welcome_msg = format!("{}{}", "Kilo editor -- version ", env!("CARGO_PKG_VERSION"));
        welcome_msg.truncate(self.columns.into());

        let line_iter = &mut lines[line_offset..].into_iter();

        for row in 0..last_row {
            Cursor::move_to(self, 0, row)?;

            if (row as usize) >= lines.len() {
                if row >= welcome_row {
                    self.write('~')?;
                }

                if row == welcome_row && lines.len() == 0 {
                    self.center_until_newline(welcome_msg.as_str(), (self.columns - 1).into())?;
                }
            } else if let Some(line) = line_iter.next() {
                if line.len() < self.columns.into() {
                    self.write_str(line.as_str())?;
                } else {
                    self.write_str(&line[..self.columns.into()])?;
                }
            }

            self.clear_until_newline()?;
        }

        Cursor::move_to(self, 0, last_row)?;

        if lines.len() < (self.max_row() as usize) {
            self.clear_from_cursor_down()?;
        } else if let Some(line) = line_iter.next() {
            if line.len() < self.columns.into() {
                self.write_str(line.as_str())?;
                self.buffer.push_str(
                    format!("{}", terminal::Clear(terminal::ClearType::FromCursorDown)).as_str()
                );
            } else {
                self.write_str(&line[..self.columns.into()])?;
            }
        }

        Ok(self)
    }

    pub fn write(&mut self, c: char) -> Result<&mut Self> {
        self.buffer.push(c);
        Ok(self)
    }

    pub fn write_str(&mut self, s: &str) -> Result<&mut Self> {
        self.buffer.push_str(s);
        Ok(self)
    }

    fn flush(&mut self) -> Result<&mut Self> {
        self.stdout.queue(Print(self.buffer.as_str()))?.flush()?;
        Ok(self)
    }

    pub fn refresh(&mut self, lines: &Vec<String>, line_offset: usize) -> Result<&mut Self> {
        Cursor::hide(self)?;
        self.draw_rows(lines, line_offset)?;

        let (cy, cx) = Cursor::cursor();
        Cursor::move_to(self, cy, cx)?;

        Cursor::show(self)?;
        self.flush()
    }

    pub fn clear(&mut self) -> Result<&mut Self> {
        self.buffer.push_str(
            format!(
                "{}{}",
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, 0)
            ).as_str()
        );
        Ok(self)
    }

    pub fn clear_until_newline(&mut self) -> Result<&mut Self> {
        self.buffer.push_str(
            format!("{}\r\n", terminal::Clear(terminal::ClearType::UntilNewLine)).as_str()
        );
        Ok(self)
    }

    pub fn clear_from_cursor_down(&mut self) -> Result<&mut Self> {
        self.buffer.push_str(
            format!("~{}", terminal::Clear(terminal::ClearType::FromCursorDown)).as_str()
        );

        Ok(self)
    }

    pub fn center_until_newline(&mut self, msg: &str, space_left: usize) -> Result<&mut Self> {
        self.buffer.push_str(
            format!("{}", msg.pad_to_width_with_alignment(space_left, Alignment::Middle)).as_str()
        );

        Ok(self)
    }
}