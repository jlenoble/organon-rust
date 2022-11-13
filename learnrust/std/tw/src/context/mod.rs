mod initialize;

use crate::Timer;

pub struct Context {
    home_dir: String,

    timer_total: Timer,
}

impl Context {
    pub fn new() -> Self {
        Self {
            home_dir: String::new(),

            timer_total: Timer::new(),
        }
    }
}