////////////////////////////////////////////////////////////////////////////////
// Converts '0'     -> 0
//          '9'     -> 9
//          'a'/'A' -> 10
//          'f'/'F' -> 15
fn xdigit(ch: u8) -> u32 {
    (if ch >= b'0' && ch <= b'9' {
        ch - b'0'
    } else if ch >= b'a' && ch <= b'f' {
        ch + 10 - b'a'
    } else if ch >= b'A' && ch <= b'F' {
        ch + 10 - b'A'
    } else {
        0
    }) as u32
}

////////////////////////////////////////////////////////////////////////////////
// Note: Assumes 4-digit hex codepoints:
//         xxxx
//         \uxxxx
//         U+xxxx
pub fn utf8_codepoint(input: &String) -> u32 {
    let mut codepoint: u32 = 0;
    let len = input.len();
    let input = input.as_bytes();

    // U+xxxx, \uxxxx
    if
        input.len() >= 6 &&
        ((input[0] == b'U' && input[1] == b'+') || (input[0] == b'\\' && input[1] == b'u'))
    {
        codepoint =
            (xdigit(input[2]) << 12) |
            (xdigit(input[3]) << 8) |
            (xdigit(input[4]) << 4) |
            xdigit(input[5]);
    } else if len >= 4 {
        codepoint =
            (xdigit(input[0]) << 12) |
            (xdigit(input[1]) << 8) |
            (xdigit(input[2]) << 4) |
            xdigit(input[3]);
    }

    return codepoint;
}

////////////////////////////////////////////////////////////////////////////////
// http://en.wikipedia.org/wiki/UTF-8
pub fn utf8_character(codepoint: u32) -> String {
    let mut sequence = String::new();
    sequence.reserve(4);
    let sequence = unsafe { sequence.as_bytes_mut() };

    // 0xxxxxxx -> 0xxxxxxx
    if codepoint < 0x80 {
        sequence[0] = codepoint as u8;
    } else if
        // 00000yyy yyxxxxxx -> 110yyyyy 10xxxxxx
        codepoint < 0x800
    {
        sequence[0] = 0xc0 | (((codepoint & 0x7c0) >> 6) as u8);
        sequence[1] = 0x80 | ((codepoint & 0x3f) as u8);
    } else if
        // zzzzyyyy yyxxxxxx -> 1110zzzz 10yyyyyy 10xxxxxx
        codepoint < 0x10000
    {
        sequence[0] = 0xe0 | (((codepoint & 0xf000) >> 12) as u8);
        sequence[1] = 0x80 | (((codepoint & 0xfc0) >> 6) as u8);
        sequence[2] = 0x80 | ((codepoint & 0x3f) as u8);
    } else if
        // 000wwwzz zzzzyyyy yyxxxxxx -> 11110www 10zzzzzz 10yyyyyy 10xxxxxx
        codepoint < 0x110000
    {
        sequence[0] = 0xf0 | (((codepoint & 0x1c0000) >> 18) as u8);
        sequence[1] = 0x80 | (((codepoint & 0x03f000) >> 12) as u8);
        sequence[2] = 0x80 | (((codepoint & 0x0fc0) >> 6) as u8);
        sequence[3] = 0x80 | ((codepoint & 0x3f) as u8);
    }

    std::str::from_utf8(sequence).unwrap().to_owned()
}