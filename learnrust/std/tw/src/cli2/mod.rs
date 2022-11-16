use crate::{ Context, File, Result };

pub struct CLI2 {}

impl CLI2 {
    pub fn get_override(args: &Vec<String>) -> Result<Option<File>> {
        let value = CLI2::get_value(args, "rc".into());
        if value.is_empty() {
            return Ok(None);
        }
        Ok(Some(File::new(value.as_str())?))
    }

    pub fn apply_overrides(args: &Vec<String>, context: &mut Context) {
        for raw in args.iter() {
            if raw == "--" {
                break;
            }
            if &raw[0..3] != "rc." {
                continue;
            }
            let pos = if let Some(pos) = raw.find(&[':', '=']) {
                pos
            } else {
                continue;
            };
            let name = &raw[3..pos];
            let value = &raw[pos + 1..];
            context.config_set(name.to_owned(), value.to_owned());
        }
    }

    pub fn get_value(args: &Vec<String>, arg: String) -> String {
        let mut after_dash_dash = false;

        for s in args.iter() {
            if after_dash_dash {
                if
                    s.len() > arg.len() + 1 &&
                    ((s.as_bytes()[arg.len()] as char) == ':' ||
                        (s.as_bytes()[arg.len()] as char) == '=') &&
                    s[0..arg.len()] == arg
                {
                    return s[arg.len() + 1..].into();
                }
            } else if s == "--" {
                after_dash_dash = true;
            }
        }

        "".into()
    }
}