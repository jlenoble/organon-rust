use crate::{ Result, Task, TaskError };

pub struct LogEntry {
    original: String,
    props: Vec<(String, String)>,
}

impl LogEntry {
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
            let (key, value, rst) = dbg!(Self::next_key_value(rest.as_str())?);

            self.props.push((key.into(), value.into()));

            if rst.is_empty() {
                break;
            }

            rest = rst;
        }

        Ok(())
    }

    #[inline]
    fn next_key_value(s: &str) -> Result<(String, String, String)> {
        let (key, rest) = Self::split_at(s, ':')?;
        let (value, rest) = match key.as_str() {
            "\"imask\"" => Self::split_at(rest.as_str(), ',')?,
            "\"annotations\"" => {
                let (k1, v1, r1) = Self::next_key_value(&rest[2..])?;
                if k1 != "entry" {
                    return Err(
                        TaskError::BadPropertyFormat(
                            s.into(),
                            "In `annotations`, first key should be `entry`".to_owned()
                        )
                    );
                }

                let (k2, value, r2) = Self::next_key_value(r1.as_str())?;
                if k2 != "description" {
                    return Err(
                        TaskError::BadPropertyFormat(
                            s.into(),
                            "In `annotations`, second key should be `description`".to_owned()
                        )
                    );
                }

                let key = format!("annotation_{}", Task::parse_datetime(&v1)?.timestamp());

                let rest = if &r2[..2] == "}," {
                    let mut r = "\"annotations\":[".to_owned();
                    r.push_str(&r2[2..]);
                    r
                } else {
                    (&r2[2..]).to_owned()
                };

                return Ok((key, value, rest));
            }
            _ => Self::split_at(&rest[1..], '"')?,
        };

        Ok(((&key[1..key.len() - 1]).to_owned(), value, rest))
    }

    #[inline]
    pub fn split_at(s: &str, ch: char) -> Result<(String, String)> {
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

                let mut rest = &s[position + 1..];

                if !rest.is_empty() && rest.as_bytes()[0] == b',' {
                    rest = &rest[1..];
                }

                return Ok(((&s[0..position]).to_owned(), rest.to_owned()));
            } else {
                break;
            }
        }

        Err(TaskError::CharacterNotFound(s.into(), ch))
    }

    #[inline]
    fn unwrap(s: &str) -> Result<String> {
        if s.as_bytes()[0] == b'{' && s.as_bytes()[s.as_bytes().len() - 1] == b'}' {
            Ok((&s[1..s.len() - 1]).to_owned())
        } else {
            Err(TaskError::BadLineFormat(s.to_owned(), "Not within brackets `{...}`".to_owned()))
        }
    }
}