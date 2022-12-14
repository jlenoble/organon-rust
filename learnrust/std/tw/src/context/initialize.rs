use std::env;
use crate::{ AsPath, CLI2, File, Result, Timer, TWError };
use super::{ CONFIGURATION_DEFAULTS, Context };

impl Context {
    pub fn initialize(&mut self, args: &Vec<String>) -> Result<()> {
        self.timer_total.start();

        self.home_dir = match env::var("HOME") {
            Ok(var) => var,
            Err(_) => {
                return Err(TWError::MissingEnvVariable("HOME".into()));
            }
        };

        ////////////////////////////////////////////////////////////////////////////
        //
        // [1] Load the correct config file.
        //     - Default to ~/.taskrc (ctor).
        //     - If no ~/.taskrc, use $XDG_CONFIG_HOME/task/taskrc if exists, or
        //       ~/.config/task/taskrc if $XDG_CONFIG_HOME is unset
        //     - Allow $TASKRC override.
        //     - Allow command line override rc:<file>
        //     - Load resultant file.
        //     - Apply command line overrides to the config.
        //
        ////////////////////////////////////////////////////////////////////////////

        let mut taskrc_overridden = false;

        // XDG_CONFIG_HOME doesn't count as an override (no warning header)
        if !self.rc_file.as_path().exists() {
            // Use XDG_CONFIG_HOME if defined, otherwise default to ~/.config
            let env_xdg_config_home = env::var("XDG_CONFIG_HOME").unwrap_or("".into());

            let mut xdg_config_home = if env_xdg_config_home.is_empty() {
                format!("{}/.config", self.home_dir)
            } else {
                format!("{}", env_xdg_config_home)
            };

            // Ensure the path does not end with '/'
            if xdg_config_home.chars().last() == Some('/') {
                xdg_config_home.pop();
            }

            // https://github.com/GothenburgBitFactory/libshared/issues/32
            let rcfile_path = format!("{}/task/taskrc", xdg_config_home);

            let maybe_rc_file = File::new(rcfile_path.as_str())?;
            if maybe_rc_file.as_path().exists() {
                self.rc_file = maybe_rc_file;
            }
        }

        let override_path = env::var("TASKRC").unwrap_or("".into());

        if !override_path.is_empty() {
            self.rc_file = File::new(override_path.as_str())?;
            taskrc_overridden = true;
        }

        if let Some(file) = CLI2::get_override(args)? {
            taskrc_overridden = true;
            self.rc_file = file;
        }

        {
            let timer = Timer::new();
            self.config.parse(CONFIGURATION_DEFAULTS)?;
            self.config.load(&self.rc_file.as_str().to_owned())?;
            Self::debug_timing(&format!("Config::load ({})", self.rc_file.as_str()), &timer);
        }

        CLI2::apply_overrides(args, self);

        if taskrc_overridden && self.verbose("override") {
            self.header(&format!("TASKRC override: {}", self.rc_file.as_str()));
        }

        Ok(())
    }
}