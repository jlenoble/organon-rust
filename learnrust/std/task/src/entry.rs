use crate::{ Result, TaskError };

pub struct Entry {
    original: String,
    props: Vec<(String, String)>,
}

impl Entry {
    pub fn new(line: String) -> Self {
        Self {
            original: line,
            props: vec![],
        }
    }

    pub fn get_props(&self) -> &Vec<(String, String)> {
        &self.props
    }

    pub fn parse(&mut self) -> Result<()> {
        if !self.props.is_empty() {
            return Err(TaskError::EntryAlreadyParsed(self.original.clone()));
        }

        let mut rest = Self::unwrap(self.original.as_str())?;

        loop {
            let (key, value, rst) = Self::next_key_value(rest)?;

            self.props.push((key.to_owned(), value.to_owned()));

            if rst.is_empty() {
                break;
            }

            rest = rst;
        }

        Ok(())
    }

    #[inline]
    fn next_key_value(s: &str) -> Result<(&str, &str, &str)> {
        let (key, rest) = Self::split_at(s, ':')?;
        let (value, rest) = Self::split_at(&rest[1..], '"')?;

        Ok((key, value, rest))
    }

    #[inline]
    pub fn split_at(s: &str, ch: char) -> Result<(&str, &str)> {
        let mut position: usize = 0;
        let mut rest = s;

        loop {
            if let Some(pos) = rest.find(ch) {
                position += pos;

                if pos > 0 && rest.as_bytes()[pos - 1] == b'\\' {
                    position += 1;
                    rest = &s[position..];
                    continue;
                }

                return Ok((&s[0..position], &s[position + 1..].trim()));
            } else {
                break;
            }
        }

        Err(TaskError::CharacterNotFound(s.to_owned(), ch))
    }

    #[inline]
    fn unwrap(s: &str) -> Result<&str> {
        if s.as_bytes()[0] == b'[' && s.as_bytes()[s.as_bytes().len() - 1] == b']' {
            Ok(&s[1..s.len() - 1])
        } else {
            Err(TaskError::BadLineFormat(s.to_owned(), "Not within brackets `[...]`".to_owned()))
        }
    }
}