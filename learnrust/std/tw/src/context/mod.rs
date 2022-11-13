mod initialize;

use crate::Timer;

pub struct Context {
    timer_total: Timer,
}

impl Context {
    pub fn new() -> Self {
        Self {
            timer_total: Timer::new(),
        }
    }
}