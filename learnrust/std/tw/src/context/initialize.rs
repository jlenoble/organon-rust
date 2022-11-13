use super::Context;

impl Context {
    pub fn initialize(&mut self, _args: &Vec<String>) {
        self.timer_total.start();
    }
}