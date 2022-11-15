mod configuration_defaults;
mod initialize;

use crate::{ Configuration, File, Result, Timer };

pub use configuration_defaults::CONFIGURATION_DEFAULTS;

pub struct Context {
    home_dir: String,
    rc_file: File,
    config: Configuration,

    timer_total: Timer,
}

impl Context {
    pub fn new() -> Result<Self> {
        Ok(Self {
            home_dir: String::new(),
            rc_file: File::new("~/.taskrc")?,
            config: Configuration::new(),

            timer_total: Timer::new(),
        })
    }
}