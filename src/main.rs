use crate::constant::NAME;
use ast::root_parse;
use colorize::AnsiColor;
use token::TokenStream;
mod ast;
mod constant;
mod token;
mod util;
static mut VERBOSE_FLAG: usize = 3;
fn main() {
    let cli_args: Vec<String> = std::env::args().collect();
    let entry_main = if let Some(s) = cli_args.get(1) {
        s
    } else {
        panic!("no file");
    };
    let mut token_stream = TokenStream::new();
    match token_stream.tokenize_source_tree(entry_main) {
        Ok(()) => (),
        Err(e) => println!("{e}"),
    };
    println!("token stream :\n {token_stream}");
    match root_parse(token_stream) {
        Ok(ast) => println!("{ast:?}"),
        Err(e) => println!("{e}"),
    };
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
