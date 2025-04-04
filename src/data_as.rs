// compiler interface for '.data' assembler section
use crate::util::{self, CompilerError};
const DATA_SECTION: &str = ".data";
pub struct DataSection {
    definitions: Vec<DataDefinition>,
}

impl DataSection {
    pub fn new() -> Self {
        Self {
            definitions: Vec::new(),
        }
    }
    pub fn add_definition(&mut self, def: DataDefinition) {
        self.definitions.push(def);
    }
    pub fn get_label(&self, label_name: &str) -> Result<&Label, CompilerError> {
        let mut label: Option<&Label> = None;
        for def in &self.definitions {
            if def.label.name == label_name {
                label = Some(&def.label)
            }
        }
        if let Some(lbl) = label {
            Ok(lbl)
        } else {
            Err(CompilerError::new(&format!(
                "referenced label [ {label_name} ] does not exist"
            )))
        }
    }

    pub fn get_definition_for_label(
        &self,
        label_name: &str,
    ) -> Result<&DataDefinition, CompilerError> {
        for entry in &self.definitions {
            if entry.label.name == label_name {
                return Ok(entry);
            }
        }
        Err(CompilerError::new(&format!(
            "referenced label [ {label_name} ] does not exist"
        )))
    }
    /// compiles data section into nisvc-as .data string
    pub fn compile(&self) -> String {
        let mut compiled: String = DATA_SECTION.to_string();
        compiled.push('\n');
        for definition in &self.definitions {
            compiled.push('\t');
            let compiled_definition = definition.compile();
            compiled.push_str(&compiled_definition);
            compiled.push('\n');
        }
        compiled
    }
}

enum WordSize {
    OneByte,
    TwoBytes,
    FourBytes,
    EightBytes,
}

pub enum DefInstr {
    Str(String),
    Res((WordSize, usize)),
    Def((WordSize, Vec<usize>)),
    Equ(usize),
}

impl DefInstr {
    /// compiles data instruction line to nisvc-as string definition
    fn compile(&self) -> String {
        let compiled_definition: String = match self {
            DefInstr::Str(string) => self._compile_str(string),
            DefInstr::Res((wordsize, length)) => self._compile_res(wordsize, length),
            DefInstr::Def((wordsize, array)) => self._compile_def(wordsize, array),
            DefInstr::Equ(literal) => self._compile_equ(literal),
        };
        let mut keyword = self.instruction_to_keyword().to_owned();
        keyword.push(' ');
        keyword + compiled_definition.as_str()
    }
    fn _compile_str(&self, string: &str) -> String {
        todo!()
    }
    fn _compile_res(&self, wordsize: &WordSize, length: &usize) -> String {
        todo!()
    }
    fn _compile_def(&self, wordsize: &WordSize, array: &[usize]) -> String {
        todo!()
    }

    fn _compile_equ(&self, literal: &usize) -> String {
        util::to_nisvc_as_int(literal)
    }

    fn instruction_to_keyword(&self) -> &str {
        match self {
            DefInstr::Str(_) => "str",
            DefInstr::Res(_) => "res",
            DefInstr::Def(_) => "def",
            DefInstr::Equ(_) => "equ",
        }
    }
}

pub struct DataDefinition {
    label: Label,
    instruction: DefInstr,
}

impl DataDefinition {
    pub fn new(label_name: &str, instruction: DefInstr) -> Self {
        let is_relative = match instruction {
            DefInstr::Str(_) => true,
            DefInstr::Res(_) => true,
            DefInstr::Def(_) => true,
            DefInstr::Equ(_) => false,
        };

        Self {
            label: Label {
                name: label_name.to_string(),
                is_relative,
            },
            instruction,
        }
    }

    pub fn compile(&self) -> String {
        let mut name = self.label.name.clone();
        name.push(' ');
        name + self.instruction.compile().as_str()
    }
}

pub struct Label {
    pub name: String,
    pub is_relative: bool,
}
