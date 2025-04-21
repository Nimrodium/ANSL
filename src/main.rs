use crate::constant::NAME;
use colorize::AnsiColor;
use token::TokenStream;
mod constant;
mod token;
mod util;
static mut VERBOSE_FLAG: usize = 3;
fn main() {
    let env_var: Vec<String> = std::env::args().collect();
    let entry_main = if let Some(s) = env_var.get(1) {
        s
    } else {
        panic!("no file");
    };
    let mut token_stream = TokenStream::new();
    match token_stream.parse_source_tree(entry_main) {
        Ok(()) => (),
        Err(e) => println!("{e}"),
    };
    println!("token stream :\n {token_stream}")
}

fn _verbose_println(msg: &str) {
    unsafe {
        if VERBOSE_FLAG >= 1 {
            println!("{NAME}: {} {}", "verbose:".yellow(), msg)
        }
    }
}
fn _very_verbose_println(msg: &str) {
    unsafe {
        if VERBOSE_FLAG >= 2 {
            println!("{NAME}: {} {}", "very-verbose:".yellow(), msg)
        }
    }
}

fn _very_very_verbose_println(msg: &str) {
    unsafe {
        if VERBOSE_FLAG >= 3 {
            println!("{NAME}: {} {}", "very-very-verbose:".yellow(), msg)
        }
    }
}

#[macro_export]
macro_rules! verbose_println {
    ($($arg:tt)*) => (crate::_verbose_println(&format!($($arg)*)));
}
#[macro_export]
macro_rules! very_verbose_println {
    ($($arg:tt)*) => (crate::_very_verbose_println(&format!($($arg)*)));
}
#[macro_export]
macro_rules! very_very_verbose_println {
    ($($arg:tt)*) => (crate::_very_very_verbose_println(&format!($($arg)*)));
}
