use crate::{
    data_as::DataSection,
    util::{self, to_nisvc_as_int, CompilerError},
};

// compiler interface for '.program' assembler section
enum SubRegister {
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    Q1,
    Q2,
    Q3,
    Q4,
    L,
    H,
    F,
}

impl SubRegister {
    fn compile(&self) -> &str {
        match &self {
            SubRegister::B1 => "b1",
            SubRegister::B4 => "b2",
            SubRegister::B2 => "b3",
            SubRegister::B3 => "b4",
            SubRegister::B5 => "b5",
            SubRegister::B6 => "b6",
            SubRegister::B7 => "b7",
            SubRegister::B8 => "b8",
            SubRegister::Q1 => "q1",
            SubRegister::Q2 => "q2",
            SubRegister::Q3 => "q3",
            SubRegister::Q4 => "q4",
            SubRegister::L => "l",
            SubRegister::H => "h",
            SubRegister::F => "f",
        }
    }
}
enum FullRegister {
    Null,
    PC,
    SP,
    RSP,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
}
impl FullRegister {
    fn compile(&self) -> &str {
        match self {
            FullRegister::Null => "null",
            FullRegister::PC => "pc",
            FullRegister::SP => "sp",
            FullRegister::RSP => "rsp",
            FullRegister::R1 => "r1",
            FullRegister::R2 => "r2",
            FullRegister::R3 => "r3",
            FullRegister::R4 => "r4",
            FullRegister::R5 => "r5",
            FullRegister::R6 => "r6",
            FullRegister::R7 => "r7",
            FullRegister::R8 => "r8",
            FullRegister::R9 => "r9",
            FullRegister::R10 => "r10",
            FullRegister::R11 => "r11",
            FullRegister::R12 => "r12",
        }
    }
}
struct Register {
    full: FullRegister,
    sub: SubRegister,
}
impl Register {
    fn new(full: FullRegister, sub: SubRegister) -> Self {
        Self { full, sub }
    }
    fn compile(&self) -> String {
        let full_str = self.full.compile();
        let sub_str = self.sub.compile();
        full_str.to_string() + sub_str
    }
}

pub enum Operand {
    Register(Register),
    Literal(usize),
    Label(String),
}

impl Operand {
    fn compile(&self, data: &DataSection) -> Result<String, CompilerError> {
        match self {
            Operand::Register(register) => Ok(register.compile()),
            Operand::Literal(literal) => Ok(to_nisvc_as_int(literal)),
            Operand::Label(label) => Self::_compile_label(label, data),
        }
    }

    fn _compile_label(label_name: &str, data: &DataSection) -> Result<String, CompilerError> {
        let label = data.get_label(label_name)?;
        let addressing_prefix = if label.is_relative {
            util::RELATIVE
        } else {
            util::ABSOLUTE
        };
        let mut compiled = addressing_prefix.to_string();
        compiled.push(util::LABEL);
        compiled.push_str(&label.name);
        Ok(compiled)
    }
}

// struct OperandContainer {
// }
// impl OperandContainer {
//     fn new(operands: Vec<Operand>) -> Self {
//         Self { operands }
//     }
//     /// checks if operand container length matches expected for opcode
//     fn is_len(&self, expected: usize) -> bool {
//         if self.operands.len() == expected {
//             true
//         } else {
//             false
//         }
//     }

// }
pub enum Opcode {
    Ldi,
    Mov,
    Load,
    Store,
}

impl Opcode {
    pub fn compile(&self) -> &str {
        match self {
            Self::Ldi => "ldi",
            Self::Mov => "mov",
            Self::Load => "load",
            Self::Store => "store",
        }
    }
}

pub struct Instruction {
    opcode: Opcode,
    operands: Vec<Operand>,
}
impl Instruction {
    pub fn new(opcode: Opcode, operands: Vec<Operand>) -> Self {
        Self { opcode, operands }
    }
    pub fn compile(&self, data: &DataSection) -> Result<String, CompilerError> {
        let mut compiled_opcode = self.opcode.compile().to_string();
        compiled_opcode.push(' ');
        let compiled_operands = self._compile_operands(data)?;
        Ok(compiled_opcode + compiled_operands.as_str())
    }
    fn _compile_operands(&self, data: &DataSection) -> Result<String, CompilerError> {
        let mut compiled = String::new();
        for operand in &self.operands {
            compiled.push_str(operand.compile(data)?.as_str());
            compiled.push(',');
        }
        Ok(compiled)
    }
}
