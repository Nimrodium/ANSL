pub const SPECIAL_CHARS: &[char] = &[
    '{', '}', '(', ')', '[', ']', ';', '=', '+', '-', '*', '/', '%', '>', '<', '!', '&', '|', '^',
    ':', '.', ',',
];
pub const PREPROCESSOR: char = '#';
pub const STR_LIT: char = '"';
pub const CHR_LIT: char = '\'';
pub const PTR: char = '&';
pub const DELIMITER: char = ';';
pub const SYSTEM_LIB: &str = "./ansl-systemlib/";
pub const IGNORE_PATTERN: &[char] = &['\'', '"'];
pub const SPLIT_PATTERN: &[char] = &[' ', '\n'];
