use std::fs::{read_to_string, File};

use {constant::SPECIAL_CHARS, preprocessor::tokenize};
// use data_as::{DataDefinition, DataSection, DefInstr};
// use program_as::{Instruction, Operand};
use util::lex_file;

mod constant;
mod data_as;
mod intermediate_backend;
mod preprocessor;
mod program_as;
mod util;
fn main() {
    let f = "../ansl-src/hello_world.ansl";
    let s = read_to_string(f).unwrap();
    let split_s = lex_file(&s, f, &['\'', '"'], &[' ', '\n'], SPECIAL_CHARS);
    println!("{split_s:#?}");
    let tokenized = tokenize(&split_s);
    // println!("{tokenized:#?}");
    // let mut data = DataSection::new();

    // for i in 0..=10 {
    //     data.add_definition(DataDefinition::new(
    //         &format!("test_label_{i}"),
    //         DefInstr::Equ(i),
    //     ));
    // }
    // let compiled = data.compile();
    // println!("{compiled}");
    // let ldi_lit = Instruction::new(program_as::Opcode::Ldi, vec![Operand::Literal(1)]);
    // let ldi_lbl = Instruction::new(
    //     program_as::Opcode::Ldi,
    //     vec![Operand::Label("test_label_1".to_string())],
    // );
    // let com_ldi_lit = ldi_lit.compile(&data).unwrap();
    // let com_ldi_lbl = ldi_lbl.compile(&data).unwrap();
    // println!("{com_ldi_lit}\n{com_ldi_lbl}")
}
