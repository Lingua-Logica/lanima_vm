use std::{collections::HashMap, rc::Rc};

use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum OpCode {
    MOVRR,
    MOVRI,
    MOVRM,
    MOVMR,
    MOVMI,
    MOVMM,
    NEWI,
    NEWSTR,
    ADDR,
    ADDM,
    ADDI,
    SUBR,
    SUBM,
    SUBI,
    MULR,
    MULI,
    MULM,
    DIVR,
    DIVI,
    DIVM,

    CALL,
    RET,
    VMCALL,

    PrintReg,

    CMP,
    JMP,
    JE,  // ==
    JNE, // !=
    JG,  // >
    JGE, // >=
    JL,  // <
    JLE, // <=

    BLE,
    BGE,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Op {
    pub op: OpCode,
    pub operands: Rc<[u8]>
}

impl Op {
    pub fn new(
        op: OpCode,
        operands: Rc<[u8]>
    ) -> Op {
        Op {
            op, operands
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CodeDef {
    pub name: &'static str,
    pub operand_widths: &'static [i32],
}

pub fn create_def(name: &'static str, operand_widths: &'static [i32]) -> CodeDef {
    CodeDef {
        name, operand_widths
    }
}

pub static CODEDEF_MAP: Lazy<HashMap<OpCode, CodeDef>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(OpCode::MOVRI, create_def("movri", &[1, 8]));
    m.insert(OpCode::MOVRR, create_def("movrr", &[1, 8]));
    m.insert(OpCode::ADDI, create_def("addi", &[1, 8]));
    m.insert(OpCode::PrintReg, create_def("printreg", &[1]));

    m
});