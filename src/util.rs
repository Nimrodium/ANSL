use string_interner::{DefaultStringInterner, DefaultSymbol};

use crate::preprocessor::Metadata;
/// extract an enclosure from a string
/// eg: splitting an array definition content from a line.
/// **usage**
/// ```
/// let array_content : str = parse_enclosure("var : arr[] [var1,var2,"ignored]",1"]",']',&[''','"'])
/// assert!(array_content)
/// - limiter : char
///     >
// fn parse_enclosure(s: &str, limiter: char, ignore_limiters: &[char]) -> String {
//     let result = String::new();
// }
use std::fmt;
pub const ABSOLUTE: char = '$';
pub const RELATIVE: char = '@';
pub const LABEL: char = '!';

/// convert an integer into a nisvc-as syntax decimal integer
pub fn to_nisvc_as_int(value: &usize) -> String {
    let str_int = value.to_string();
    let format = "$d";
    (format.to_string() + str_int.as_str())
}
pub struct CompilerError {
    err: String,
}
impl CompilerError {
    pub fn new(err: &str) -> Self {
        Self {
            err: err.to_string(),
        }
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error during compilation: {}", self.err)
    }
}
impl fmt::Debug for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
// pub type LexxedSource<'a> = ;
pub struct LexxedSource {
    pub inner: Vec<Vec<(DefaultSymbol, Metadata)>>,
    pub str_db: DefaultStringInterner,
}

impl LexxedSource {
    fn new(str_db: DefaultStringInterner) -> Self {
        Self {
            inner: Vec::new(),
            str_db,
        }
    }
    fn new_lex(
        &mut self,
        lexeme: &str,
        file: &str,
        line: &str,
        line_number: usize,
        line_column: usize,
    ) {
        let metadata = Metadata::new(
            self.str_db.get_or_intern(file),
            self.str_db.get_or_intern(line),
            line_number,
            line_column,
        );
        let interned_lexeme = self.str_db.get_or_intern(lexeme);
        let current_line = if let Some(l) = self.inner.last_mut() {
            l
        } else {
            self.new_line();
            self.inner.last_mut().unwrap()
        };
        current_line.push((interned_lexeme, metadata));
    }
    fn delete_last_lex(&mut self) {
        if let Some(l) = self.inner.last_mut() {
            l.pop();
        }
    }

    fn new_line(&mut self) {
        self.inner.push(Vec::new())
    }
}

pub fn lex_file<'a>(
    s: &'a str,
    file_path: &'a str,
    ignore_pattern: &[char],
    split_pattern: &[char],
    special_split_pattern: &[char], // split and include
    str_db: DefaultStringInterner,
) -> LexxedSource {
    let mut lexxed_source = LexxedSource::new(str_db);

    let mut ignore: bool = false;

    let mut buf: String = String::new();
    let s_lines = s.split('\n');

    let mut line_number = 0;
    let mut line_column = 0;
    let mut line_column_start = 0;
    for line in s_lines {
        let mut skip_comment = false;
        let mut last_char_was_start_of_comment = false;
        line_column = 0;
        line_number += 1;
        for c in line.chars() {
            if skip_comment {
                continue;
            }
            line_column += 1;
            if !ignore {
                if last_char_was_start_of_comment && c == '/' {
                    skip_comment = true;
                    lexxed_source.delete_last_lex();
                    continue;
                }
                if c == '/' {
                    println!("maybe comment");
                    last_char_was_start_of_comment = true;
                } else {
                    last_char_was_start_of_comment = false;
                }
                match c {
                    c if ignore_pattern.contains(&c) => {
                        ignore = true;
                        buf.push(c);
                        continue;
                    }
                    c if split_pattern.contains(&c) => {
                        if !buf.is_empty() {
                            lexxed_source.new_lex(&buf, file_path, line, line_number, line_column);
                            buf.clear();
                            line_column_start = line_column;
                        }
                    }

                    c if special_split_pattern.contains(&c) => {
                        if !buf.is_empty() {
                            lexxed_source.new_lex(&buf, file_path, line, line_number, line_column);
                            buf.clear();
                            line_column_start = line_column + 1;
                        }

                        buf.push(c);
                        lexxed_source.new_lex(&buf, file_path, line, line_number, line_column);
                        buf.clear();
                        line_column_start = line_column + 1;
                    }

                    _ => {
                        buf.push(c);
                    }
                }
            } else {
                if ignore_pattern.contains(&c) {
                    ignore = false;
                    buf.push(c);
                    continue;
                } else {
                    buf.push(c);
                }
            }
        }
        if !buf.is_empty() {
            lexxed_source.new_lex(&buf, file_path, line, line_number, line_column);
            buf.clear();
        }
    }

    lexxed_source
}
