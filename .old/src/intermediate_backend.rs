use std::collections::HashMap;

use crate::util::CompilerError;

// from AST to AS
type SSAID = usize;
enum Operation {
    Ldi,
    Mov,
    Load,
    Store,
    Add,
    Sub,
    Mult,
    Div,
}

struct LogicalInstruction {
    operation: Operation,
    outputs: Vec<SSAValue>,
    inputs: Vec<SSAID>,
}

/// holds logical instructions
struct LogicalIntermediateBlock {
    instructions: Vec<LogicalInstruction>,
}

enum SSAValueSize {
    EightBit,
    SixteenBit,
    ThirtytwoBit,
    SixtyfourBit,
}

#[derive(PartialEq)]
enum SSAValueState {
    Unbound,
    Active,
    Spilled,
    Expired,
}

/// SSA atomic value
pub struct SSAValue {
    id: SSAID,
    size: SSAValueSize,
    usage_stamps: Vec<usize>,
    last_use: Option<usize>,
    state: SSAValueState,
}

impl SSAValue {
    pub fn new(id: SSAID, size: SSAValueSize) -> Self {
        Self {
            id,
            state: SSAValueState::Unbound,
            size,
            usage_stamps: Vec::new(),
            last_use: None,
        }
    }
    pub fn get_next_use(current_position: usize) -> Result<usize, CompilerError> {
        todo!()
    }
}

struct SpilledSSAValue {
    ssa_value: SSAID,
    stack_offset: usize,
}
impl SpilledSSAValue {
    fn new(id: usize) -> Self {
        Self {
            ssa_value: id,
            stack_offset: 0,
        }
    }
}
type VRID = usize;
struct VirtualAssemblyInstruction {
    operation: Operation,
    outputs: Vec<VRID>,
    inputs: Vec<VRID>,
}

/// assembly code with registers bound but not yet finalized.
struct VirtualAssemblyBlock {}

/// local compiler for a single scope
struct VirtualRegisterResolver {
    ssa_values: HashMap<SSAID, SSAValue>,
    virtual_registers: HashMap<VRID, VirtualRegister>,
    ssa_register_mapping: HashMap<SSAID, VRID>,
    spilled_ssa_values: Vec<SpilledSSAValue>,
    code: LogicalIntermediateBlock,
    line: usize,
    free_virtual_register_queue: Vec<VRID>,
}

impl VirtualRegisterResolver {
    fn new(ssa_values: Vec<SSAValue>, code: LogicalIntermediateBlock) -> Self {
        todo!()
    }

    fn compile(&mut self) {}

    // for each line
    // update free register stack
    // read instruction
    // look for requested SSAs
    // if SSA active, assign its current bound register
    // if SSA spilled, look for SSA that isnt needed for the longest and spill it, loading in the newly freed register
    // if SSA expired raise error
    // check if operands die in this operation, and free their registers if the do
    // assign result registers based on next available free registers
    //
    fn step(&mut self) -> Result<(), CompilerError> {
        let working_instruction = match self.code.instructions.get(self.line) {
            Some(i) => i,
            None => {
                return Err(CompilerError::new(&format!(
                    "instruction {} does not exist",
                    self.line
                )))
            }
        };
        for input_ssa in &working_instruction.inputs {
            let ssa = self.get_ssa_value(input_ssa)?;
            // check location and move as needed
        }
        Ok(())
    }
    // retrieves a free register to store a value
    fn pop_vr(&mut self) {}

    // returns a virtual register which contains the ssa. if none are found which already contain this value
    //  then a register will be spilled to make room
    fn retrieve_ssa_register(&mut self, id: &SSAID) -> Result<VRID, CompilerError> {
        todo!()
    }

    fn get_ssa_value(&self, id: &SSAID) -> Result<&SSAValue, CompilerError> {
        let ssa = match self.ssa_values.get(&id) {
            Some(id) => id,
            None => return Err(CompilerError::new(&format!("SSA::{id} does not exist"))),
        };
        if ssa.state == SSAValueState::Expired {
            return Err(CompilerError::new(&format!("SSA::{id} has expired")));
        }
        Ok(ssa)
    }
    fn get_mut_ssa_value(&mut self, id: SSAID) -> Result<&mut SSAValue, CompilerError> {
        let ssa = match self.ssa_values.get_mut(&id) {
            Some(id) => id,
            None => return Err(CompilerError::new(&format!("SSA::{id} does not exist"))),
        };
        if ssa.state == SSAValueState::Expired {
            return Err(CompilerError::new(&format!("SSA::{id} has expired")));
        }
        Ok(ssa)
    }

    fn update_spilled_ssa_value_after_push_event(&mut self) {
        for spilled_ssa_value in &mut self.spilled_ssa_values {
            spilled_ssa_value.stack_offset += 1;
        }
    }

    /// spill register and enter ssa_value into compiler tracker, and
    fn spill_register(&mut self, id: usize) -> Result<(), CompilerError> {
        let register = match self.virtual_registers.get_mut(&id) {
            Some(r) => r,
            None => {
                return Err(CompilerError::new(&format!(
                    "virtual register {id} does not exist yet was called to spill"
                )))
            }
        };

        let spilled_luid_tracker = register.spill()?;
        self.spilled_ssa_values.push(spilled_luid_tracker);

        Ok(())
    }
}
/// manages updating registers
// struct RegisterWorker {}
#[derive(PartialEq)]
enum RegisterState {
    Bound,
    Free,
}

/// a virtual register which can hold a singular LUID or be tied to a LUID's lifetime, without being bound.
/// - id : identifier for the virtual register
/// - luid : LUID currently bound
/// - lifetime : lifetime bound to the register, will be the LUID's lifetime if a luid is bound.
/// - state : whether the register is bound or free
///
/// a lifetime id is the same as the LUID.
struct VirtualRegister {
    id: usize,
    ssa_value: Option<SSAID>,
    lifetime: Option<SSAID>,
    state: RegisterState,
}

impl VirtualRegister {
    fn new(id: usize) -> Self {
        Self {
            id,
            ssa_value: None,
            lifetime: None,
            state: RegisterState::Free,
        }
    }

    fn bind_ssa_value(&mut self, ssa_value: SSAID) -> Result<(), CompilerError> {
        if self.state == RegisterState::Bound {
            return Err(CompilerError::new(&format!("attempted to bind LUID::{ssa_value} to virtual register {}, however virtual register was already bound",self.id)));
        }
        self.ssa_value = Some(ssa_value);
        self.lifetime = Some(ssa_value);
        self.state = RegisterState::Bound;
        Ok(())
    }

    fn bind_lifetime(&mut self, lifetime: SSAID) -> Result<(), CompilerError> {
        if self.state == RegisterState::Bound {
            return Err(CompilerError::new(&format!("attempted to bind lifetime associated with LUID::{lifetime} to virtual register {} however it was already bound",self.id)));
        }
        self.lifetime = Some(lifetime);

        Ok(())
    }

    // fn is_bound(&self) -> bool {
    //     self.state == RegisterState::Bound
    // }

    fn reset(&mut self) {
        self.ssa_value = None;
        self.lifetime = None;
        self.state = RegisterState::Free;
    }
    /// wrap luid into a spilled luid tracker object and reset register
    fn spill(&mut self) -> Result<SpilledSSAValue, CompilerError> {
        if self.state == RegisterState::Free {
            return Err(CompilerError::new(&format!(
                "attempted to spill virtual register {}, however it did not contain a LUID",
                self.id
            )));
        }
        let luid = if let Some(ld) = self.ssa_value {
            ld
        } else {
            panic!(
                "state misalignment :: register reported bound state when no ssa_value was bound"
            )
        };

        let spilled_luid_tracker = SpilledSSAValue::new(luid);
        self.reset();
        Ok(spilled_luid_tracker)
    }
}
