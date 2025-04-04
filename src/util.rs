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
