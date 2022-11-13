use std::env;
use crate::{ Result, TWError };
use super::Context;

impl Context {
    pub fn initialize(&mut self, _args: &Vec<String>) -> Result {
        self.timer_total.start();

        self.home_dir = match env::var("HOME") {
            Ok(var) => var,
            Err(_) => {
                return Err(TWError::MissingEnvVariable("HOME"));
            }
        };

        Ok(())
    }
}