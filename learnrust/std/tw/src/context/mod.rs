mod initialize;

use crate::{ File, Result, Timer };

pub struct Context {
    home_dir: String,
    rc_file: File,

    timer_total: Timer,
}

impl Context {
    pub fn new() -> Result<Self> {
        Ok(Self {
            home_dir: String::new(),
            rc_file: File::new("~/.taskrc")?,

            timer_total: Timer::new(),
        })
    }
}