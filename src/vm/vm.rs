use crate::{
    err::vm_err::VmError,
    object::str_val::StrVal,
    vm::{
        builtins::BUILTINS,
        code::op_code::{Op, OpCode},
        constants::{REG_COUNT, STACK_SIZE},
        heap::{Heap, heap_object::HeapVal},
        utils::read_i64,
    },
};

pub type VmResult<T> = Result<T, VmError>;

pub struct Vm<'ins> {
    program: &'ins [Op],
    pc: usize, // program counter

    stack: [usize; STACK_SIZE as usize],
    sp: usize,

    return_address_stack: Vec<usize>,

    pub reg: [i64; REG_COUNT as usize],
    pub heap: Heap,
}

impl<'ins> Vm<'ins> {
    pub fn new(program: &'ins [Op]) -> Vm<'ins> {
        Vm {
            program,
            pc: 0,

            stack: [0; STACK_SIZE as usize],
            sp: 0,

            return_address_stack: vec![],

            reg: [0; REG_COUNT as usize],
            heap: Heap::new(),
        }
    }

    pub fn next(&mut self, op: &Op) -> VmResult<()> {
        match op.op {
            OpCode::MOVRI => {
                // reg_index
                let reg = op.operands[0] as usize;

                let imm = read_i64(&op.operands[1..]);

                self.reg[reg] = imm;
            }

            OpCode::MOVRR => {
                // reg_index
                let r0 = op.operands[0] as usize;
                let r1 = op.operands[1] as usize;

                self.reg[r0] = self.reg[r1];
            }

            OpCode::ADDI => {
                // reg_index
                let r0 = op.operands[0] as usize;

                let imm = read_i64(&op.operands[1..]);

                self.reg[r0] += imm;
            }

            OpCode::SUBI => {
                // reg_index
                let r0 = op.operands[0] as usize;

                let imm = read_i64(&op.operands[1..]);

                self.reg[r0] -= imm;
            }

            OpCode::MULI => {
                // reg_index
                let r0 = op.operands[0] as usize;

                let imm = read_i64(&op.operands[1..]);

                self.reg[r0] *= imm;
            }

            OpCode::DIVI => {
                // reg_index
                let r0 = op.operands[0] as usize;

                let imm = read_i64(&op.operands[1..]);

                if imm == 0 {
                    return Err(VmError::new("division by zero"));
                }

                self.reg[r0] /= imm;
            }

            OpCode::ADDR => {
                // reg_index
                let r0 = op.operands[0] as usize;

                let r1 = op.operands[1] as usize;

                self.reg[r0] += self.reg[r1];
            }

            OpCode::SUBR => {
                // reg_index
                let r0 = op.operands[0] as usize;

                let r1 = op.operands[1] as usize;

                self.reg[r0] -= self.reg[r1];
            }

            OpCode::MULR => {
                // reg_index
                let r0 = op.operands[0] as usize;

                let r1 = op.operands[1] as usize;

                self.reg[r0] *= self.reg[r1];
            }

            OpCode::DIVR => {
                // reg_index
                let r0 = op.operands[0] as usize;

                let r1 = op.operands[1] as usize;

                if self.reg[r1] == 0 {
                    return Err(VmError::new("division by zero"));
                }

                self.reg[r0] /= self.reg[r1];
            }

            OpCode::NEWSTR => {
                let r0 = op.operands[0] as usize;

                let s = StrVal(String::from_utf8_lossy(&op.operands[1..]).to_string());

                self.reg[r0] = self.heap.push(HeapVal::StrVal(s)) as i64;
            }

            OpCode::VMCALL => {
                let call_func_index = op.operands[0] as usize;

                BUILTINS[call_func_index](self)
                    .map_err(|it| VmError::from(it.to_string().as_ref()))?
            }

            OpCode::PrintReg => {
                // reg_index
                let reg = op.operands[0] as usize;

                println!("REG{reg}: {}", self.reg[reg]);
            }

            _ => todo!(),
        }

        Ok(())
    }

    pub fn run(&mut self) -> VmResult<()> {
        while self.pc < self.program.len() {
            self.next(&self.program[self.pc])?;

            self.pc += 1;
        }

        Ok(())
    }
}
