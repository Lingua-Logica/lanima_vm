use std::{collections::HashMap, rc::Rc};

use bincode::{Decode, Encode};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode)]
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Decode, Encode)]
pub struct Op {
    pub op: OpCode,
    pub operands: Rc<[u8]>,
}

impl Op {
    pub fn new(op: OpCode, operands: Rc<[u8]>) -> Op {
        Op { op, operands }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CodeDef {
    pub name: &'static str,
    pub operand_widths: &'static [i32],
    pub infinite_operands: bool,
}

pub const fn create_def(name: &'static str, operand_widths: &'static [i32]) -> CodeDef {
    CodeDef {
        name,
        operand_widths,
        infinite_operands: false,
    }
}

pub const fn create_def_infinite_operands(
    name: &'static str,
    operand_widths: &'static [i32],
) -> CodeDef {
    CodeDef {
        name,
        operand_widths,
        infinite_operands: true,
    }
}

pub static CODEDEF_MAP: Lazy<HashMap<OpCode, CodeDef>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(OpCode::MOVRI, create_def("MOVRI", &[1, 8]));
    m.insert(OpCode::MOVRR, create_def("MOVRR", &[1, 1]));
    m.insert(OpCode::ADDR, create_def("ADDR", &[1, 1]));
    m.insert(OpCode::SUBR, create_def("SUBR", &[1, 1]));
    m.insert(OpCode::MULR, create_def("MULR", &[1, 1]));
    m.insert(OpCode::DIVR, create_def("DIVR", &[1, 1]));
    m.insert(OpCode::ADDI, create_def("ADDI", &[1, 8]));
    m.insert(OpCode::SUBI, create_def("SUBI", &[1, 8]));
    m.insert(OpCode::MULI, create_def("MULI", &[1, 8]));
    m.insert(OpCode::DIVI, create_def("DIVI", &[1, 8]));
    m.insert(OpCode::NEWSTR, create_def_infinite_operands("NEWSTR", &[1, 4]));
    m.insert(OpCode::VMCALL, create_def_infinite_operands("VMCALL", &[1]));
    m.insert(OpCode::PrintReg, create_def("PrintReg", &[1]));

    m
});
