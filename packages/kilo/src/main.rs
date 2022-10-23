use crossterm::{
    cursor,
    event::{ Event::Key, KeyCode, KeyEvent, KeyModifiers, poll, read },
    QueueableCommand,
    Result,
    terminal,
};
use errno::errno;
use std::{ io::{ stdout, Write }, process, time::Duration };

fn die<S: Into<String>>(message: S) -> ! {
    let eno = errno().to_string();

    if eno == "Success" {
        eprintln!("\r{}", message.into());
    } else {
        eprintln!("\r{} - errno = {}", message.into(), eno);
    }

    exit(1);
}

fn exit(code: i32) -> ! {
    if let Err(e) = terminal::disable_raw_mode() {
        eprintln!("failed to disable terminal raw mode: {:?}", e);
        process::exit(1);
    }

    if let Err(e) = clear_terminal() {
        die(format!("failed to clear terminal: {:?}", e));
    }

    process::exit(code);
}

fn editor_read_key() -> KeyEvent {
    if let Ok(Key(event)) = read() {
        event
    } else {
        die("failed to read terminal");
    }
}

fn editor_process_keypress() {
    let key_event = editor_read_key();

    match key_event {
        KeyEvent { code: KeyCode::Char('q'), modifiers: KeyModifiers::CONTROL, .. } => {
            exit(0);
        }
        KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::NONE, .. } => {
            println!("{}", c);
        }
        KeyEvent { code: KeyCode::Char(c), modifiers: KeyModifiers::SHIFT, .. } => {
            println!("{}", c);
        }
        _ => {
            die(format!("unhandled key event: {key_event:?}"));
        }
    }
}

fn clear_terminal() -> Result<()> {
    stdout()
        .queue(terminal::Clear(terminal::ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .flush()?;
    Ok(())
}

fn main() {
    if let Err(e) = terminal::enable_raw_mode() {
        die(format!("failed to enable terminal raw mode: {:?}", e));
    }

    if let Err(e) = clear_terminal() {
        die(format!("failed to clear terminal: {:?}", e));
    }

    loop {
        match poll(Duration::from_millis(100)) {
            Ok(true) => {
                editor_process_keypress();
            }
            Ok(false) => {}
            Err(e) => {
                die(format!("failed to poll terminal: {:?}", e));
            }
        }
    }
}