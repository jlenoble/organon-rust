use crossterm::{
    cursor,
    event::{ Event::Key, KeyCode, KeyEvent, KeyModifiers, read },
    QueueableCommand,
    Result,
    terminal,
};
use errno::errno;
use std::{ io::{ stdout, Stdout, Write }, process };

pub struct Editor {
    columns: u16,
    rows: u16,
    stdout: Stdout,
}

impl Editor {
    pub fn new(stdout: Stdout) -> Self {
        Editor::init(stdout)
    }

    fn init(stdout: Stdout) -> Editor {
        let editor = Editor { columns: 1, rows: 1, stdout };

        if let Err(e) = terminal::enable_raw_mode() {
            editor.die(format!("failed to enable terminal raw mode: {:?}", e));
        }

        if let Err(e) = editor.clear_terminal() {
            // don't use die here in order to prevent infinite recursive calls
            // as the terminal is cleared on exit.
            eprintln!("failed to clear terminal: {:?}", e);
            editor.exit(1);
        }

        match terminal::size() {
            Ok((columns, rows)) => { Editor { columns, rows, stdout: editor.stdout } }
            Err(e) => {
                editor.die(format!("could not determine terminal size: {:?}", e));
            }
        }
    }

    pub fn die<S: Into<String>>(&self, message: S) -> ! {
        if let Err(e) = self.clear_terminal() {
            // don't use die here in order to prevent infinite recursive calls
            // as the terminal is cleared on exit.
            eprintln!("failed to clear terminal: {:?}", e);
            eprintln!("\rerror die was called upon:\r");
        }

        let eno = errno().to_string();

        if eno == "Success" {
            eprintln!("{}", message.into());
        } else {
            eprintln!("{} - errno = {}", message.into(), eno);
        }

        self.exit(1);
    }

    fn clear_and_exit(&self) {
        if let Err(e) = self.clear_terminal() {
            // don't use die here in order to prevent infinite recursive calls
            // as the terminal is cleared on exit.
            eprintln!("failed to clear terminal: {:?}", e);
            eprintln!("\rerror die was called upon:\r");
            self.exit(1);
        }
        self.exit(0);
    }

    fn exit(&self, code: i32) -> ! {
        if let Err(e) = terminal::disable_raw_mode() {
            eprintln!("\rfailed to disable terminal raw mode: {:?}", e);
            process::exit(1);
        }

        process::exit(code);
    }

    fn read_key(&self) -> KeyEvent {
        if let Ok(Key(event)) = read() {
            event
        } else {
            self.die("failed to read terminal");
        }
    }

    pub fn process_keypress(&self) {
        let key_event = self.read_key();

        match key_event {
            KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, .. } => {
                self.clear_and_exit();
            }
            KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::NONE, .. } => {
                println!("{}", c);
            }
            KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::SHIFT, .. } => {
                println!("{}", c);
            }
            _ => {
                self.die(format!("unhandled key event: {key_event:?}"));
            }
        }
    }

    fn draw_rows(&self) -> Result<()> {
        let mut stdout = stdout();
        let rows = self.rows - 1;

        for _ in 0..rows {
            write!(stdout, "~\r\n")?;
        }

        write!(stdout, "~")?;
        stdout.flush()?;

        Ok(())
    }

    pub fn refresh_screen(&self) -> Result<()> {
        self.clear_terminal()?;
        self.draw_rows()?;
        self.move_cursor(0, 0)
    }

    fn move_cursor(&self, col: u16, row: u16) -> Result<()> {
        stdout().queue(cursor::MoveTo(col, row))?;
        Ok(())
    }

    fn clear_terminal(&self) -> Result<()> {
        stdout()
            .queue(terminal::Clear(terminal::ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .flush()?;
        Ok(())
    }
}