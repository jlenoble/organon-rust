use crate::{ utf8_character, utf8_codepoint };

pub fn decode(input: &String) -> String {
    let mut output = String::new();
    output.reserve(input.len());

    let mut pos: usize = 0;

    while pos < input.len() {
        if (input.as_bytes()[pos] as char) == '\\' {
            pos += 1;
            match input.as_bytes()[pos] as char {
                // Simple translations.
                '"' => output.push('"'),
                '\\' => output.push('\\'),
                '/' => output.push('/'),
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                't' => output.push('\t'),

                // Compose a UTF8 unicode character.
                'u' => {
                    pos += 1;
                    output.push_str(
                        utf8_character(utf8_codepoint(&input[pos..pos + 4].to_owned())).as_str()
                    );
                    pos += 3;
                }

                // If it is an unrecognized sequence, do nothing.
                ch => {
                    output.push('\\');
                    output.push(ch);
                }
            }
            pos += 1;
        } else if let Some(next_backslash) = input[pos..].find('\\') {
            output.push_str(&input[pos..pos + next_backslash]);
            pos += next_backslash;
        } else {
            output.push_str(&input[pos..]);
            break;
        }
    }

    return output;
}