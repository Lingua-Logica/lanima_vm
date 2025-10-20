use crate::vm::{constants::REG_COUNT, err::VmError, code::op_code::{Op, OpCode}, utils::read_i64};

pub type VmResult<T> = Result<T, VmError>;

pub struct Vm<'ins> {
    program: &'ins [Op],
    pc: usize, // program counter
    
    stack: Vec<usize>,
    sp: usize,

    return_address_stack: Vec<usize>,

    reg: [i64; REG_COUNT as usize]
}

impl<'ins> Vm<'ins> {
    pub fn new(
        program: &'ins [Op]
    ) -> Vm<'ins> {
        Vm {
            program,
            pc: 0,

            stack: vec![],
            sp: 0,

            return_address_stack: vec![],

            reg: [0; REG_COUNT as usize]
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

            OpCode::PrintReg => {
                // reg_index
                let r0 = op.operands[0] as usize;

                println!("REG{r0}: {}", self.reg[r0]);
            }

            _ => todo!()
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