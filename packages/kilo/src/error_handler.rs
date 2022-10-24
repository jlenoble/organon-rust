use errno::errno;
use crossterm::terminal;
use std::process;
use crate::screen::Screen;

#[derive(Clone, Copy)]
pub struct ErrorHandler {}

impl ErrorHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn die<S: Into<String>>(&self, screen: &mut Screen, message: S) -> ! {
        if let Err(e) = screen.clear() {
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

    pub fn exit(&self, code: i32) -> ! {
        if let Err(e) = terminal::disable_raw_mode() {
            eprintln!("\rfailed to disable terminal raw mode: {:?}", e);
            process::exit(1);
        }

        process::exit(code);
    }

    pub fn clear_and_exit(&self, screen: &mut Screen) {
        if let Err(e) = screen.clear() {
            // don't use die here in order to prevent infinite recursive calls
            // as the terminal is cleared on exit.
            eprintln!("failed to clear terminal: {:?}", e);
            eprintln!("\rerror die was called upon:\r");
            self.exit(1);
        }
        self.exit(0);
    }
}