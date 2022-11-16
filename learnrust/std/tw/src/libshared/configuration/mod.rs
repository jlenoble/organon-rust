use std::collections::HashMap;

use crate::{ AsPath, File, json, Result, TWError };

pub struct Configuration {
    _hash_map: HashMap<String, String>,
    _dirty: bool,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            _hash_map: HashMap::new(),
            _dirty: false,
        }
    }

    pub fn as_hash_map(&self) -> &HashMap<String, String> {
        &self._hash_map
    }

    pub fn as_hash_map_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self._hash_map
    }

    pub fn parse(&mut self, input: &str) -> Result<()> {
        // Shortcut case for default constructor.
        if input.is_empty() {
            return Ok(());
        }

        for line in input.lines() {
            // Remove comments.
            let line = if let Some(pound) = line.find('#') { &line[0..pound] } else { line };
            let line = line.trim();

            // Skip empty lines
            if line.is_empty() {
                continue;
            }

            // Handle key=value lines
            if let Some(equal) = line.find('=') {
                let key = line[0..equal].trim();
                let value = line[equal + 1..].trim();
                let value = shellexpand
                    ::full(value)
                    .or(Err(TWError::BadConfigEntry(line.to_owned())))?;

                self.as_hash_map_mut().insert(key.to_owned(), json::decode(&value.into_owned()));
                continue;
            }
        }

        self._dirty = true;

        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Read the Configuration file and populate the *this map.  The file format is
    // simply lines with name=value pairs.  Whitespace between name, = and value is
    // not tolerated, but blank lines and comments starting with # are allowed.
    pub fn load(&mut self, file_path: &String) -> Result<()> {
        // Read the file, then parse the contents.
        let config = File::new(file_path)?;

        if config.as_path().exists() && config.as_path().readable()? {
            let contents = config.read()?;
            self.parse(contents.as_str())?;
        }

        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////
    // Return the configuration value given the specified key.
    pub fn get(&self, key: &str, get_context_from: Option<bool>) -> String {
        let get_context_from = get_context_from.unwrap_or(false);

        let mut ckey = key.to_owned();

        if get_context_from {
            if let Some(ctx_value) = self.as_hash_map().get("context") {
                if let Some(pos) = key.rfind("context.") {
                    if pos > 0 {
                        ckey.clear();
                        ckey.push_str("context.");
                        ckey.push_str(ctx_value);
                        ckey.push_str(".rc.");
                        ckey.push_str(key);
                    }
                }
            }
        }

        if let Some(found) = self.as_hash_map().get(&ckey) {
            return found.into();
        }

        // Fallback - use global config value
        if let Some(found) = self.as_hash_map().get(key.into()) {
            return found.into();
        }

        return "".into();
    }

    ////////////////////////////////////////////////////////////////////////////////
    pub fn get_boolean(&self, key: &str, get_context_from: Option<bool>) -> bool {
        let val = self.get(key, get_context_from);
        if !val.is_empty() {
            let value = val.to_lowercase();
            if value == "true" || value == "1" || value == "y" || value == "yes" || value == "on" {
                return true;
            }
        }

        return false;
    }

    pub fn set(&mut self, key: String, value: String) {
        self.as_hash_map_mut().insert(key, value);
        self._dirty = true;
    }
}