use std::collections::HashMap;

use crate::{ json, Result, TWError };

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
}