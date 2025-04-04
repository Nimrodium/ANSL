use data_as::{DataDefinition, DataSection, DefInstr};
use program_as::{Instruction, Operand};

mod data_as;
mod intermediate;
mod program_as;
mod util;

fn main() {
    let mut data = DataSection::new();

    for i in 0..=10 {
        data.add_definition(DataDefinition::new(
            &format!("test_label_{i}"),
            DefInstr::Equ(i),
        ));
    }
    let compiled = data.compile();
    println!("{compiled}");
    let ldi_lit = Instruction::new(program_as::Opcode::Ldi, vec![Operand::Literal(1)]);
    let ldi_lbl = Instruction::new(
        program_as::Opcode::Ldi,
        vec![Operand::Label("test_label_1".to_string())],
    );
    let com_ldi_lit = ldi_lit.compile(&data).unwrap();
    let com_ldi_lbl = ldi_lbl.compile(&data).unwrap();
    println!("{com_ldi_lit}\n{com_ldi_lbl}")
}
