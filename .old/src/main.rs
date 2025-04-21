use std::{
    fs::{read_to_string, File},
    process::exit,
};

use constant::{IGNORE_PATTERN, SPLIT_PATTERN};
use preprocessor::tokenize_file;
use string_interner::DefaultStringInterner;
use {constant::SPECIAL_CHARS, preprocessor::tokenize};
// use data_as::{DataDefinition, DataSection, DefInstr};
// use program_as::{Instruction, Operand};
use util::lex_file;
mod ast;
mod constant;
mod data_as;
mod intermediate_backend;
mod preprocessor;
mod program_as;
mod util;

fn main() {
    let f = "../ansl-src/hello_world.ansl";
    let mut master_token_stream = match tokenize_file(f, DefaultStringInterner::new()) {
        Ok(ts) => ts,
        Err(err) => {
            println!("{err}");
            exit(0)
        }
    };
    println!("{master_token_stream}");
}
