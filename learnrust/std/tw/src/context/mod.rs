mod configuration_defaults;
mod initialize;
mod verbose;

use std::collections::HashSet;

use crate::{ Configuration, File, Result, Timer };

pub use configuration_defaults::CONFIGURATION_DEFAULTS;

pub struct Context {
    home_dir: String,
    rc_file: File,
    config: Configuration,

    timer_total: Timer,
    verbosity_legacy: bool,
    verbosity: HashSet<String>,
    headers: Vec<String>,
}

impl Context {
    pub fn new() -> Result<Self> {
        Ok(Self {
            home_dir: String::new(),
            rc_file: File::new("~/.taskrc")?,
            config: Configuration::new(),

            timer_total: Timer::new(),
            verbosity_legacy: false,
            verbosity: HashSet::new(),
            headers: vec![],
        })
    }

    pub fn config_set(&mut self, key: String, value: String) {
        self.config.set(key, value);
    }
}