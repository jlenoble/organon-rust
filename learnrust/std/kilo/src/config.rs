use std::env;

pub struct Config {
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => {
                return Err("Didn't get a file path");
            }
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { file_path, ignore_case })
    }
}