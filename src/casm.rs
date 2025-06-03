use std::fmt::{self, Debug, Display};

#[derive(Clone)]
pub enum CasmInstruction {
    Ret,
    Call(String),
    CallRel(i32),
    CallAbs(u64),
    IncrFp(u64),
    IncrAp(u64),
    Label(String),
    Set {
        left: Operand,
        op: Operand,
        incr_ap: bool,
    },
    Add {
        left: Operand,
        op1: Operand,
        op2: Operand,
    },
    Mul {
        left: Operand,
        op1: Operand,
        op2: Operand,
    },
    Deref {
        left: Operand,
        op: Operand,
    },
    Jmp(String),
    JmpRel(i32),
    JmpIfNeq(String, Operand),
    JmpIfNeqRel(i32, Operand),
}

#[derive(Clone)]
pub enum Operand {
    Int(u64),
    DerefFp(i32),
    DerefAp(i32),
}

impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Int(n) => write!(f, "{}", n),
            Operand::DerefFp(offset) => write!(f, "[fp + {}]", offset),
            Operand::DerefAp(offset) => write!(f, "[ap + {}]", offset),
        }
    }
}

impl Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Int(n) => write!(f, "{}", n),
            Operand::DerefFp(offset) => write!(f, "[fp + {}]", offset),
            Operand::DerefAp(offset) => write!(f, "[ap + {}]", offset),
        }
    }
}

impl Debug for CasmInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CasmInstruction::Ret => write!(f, "ret;"),
            CasmInstruction::Call(label) => write!(f, "call {};", label),
            CasmInstruction::CallRel(offset) => write!(f, "call rel {};", offset),
            CasmInstruction::CallAbs(offset) => write!(f, "call abs {};", offset),
            CasmInstruction::IncrFp(n) => write!(f, "fp += {};", n),
            CasmInstruction::IncrAp(n) => write!(f, "ap += {};", n),
            CasmInstruction::Label(label) => write!(f, "{}:", label),
            CasmInstruction::Set { left, op, incr_ap } => {
                write!(
                    f,
                    "{} = {}{}",
                    left,
                    op,
                    if *incr_ap { ", ap++;" } else { ";" }
                )
            }
            CasmInstruction::Add { left, op1, op2 } => {
                write!(f, "{} = {} + {}, ap++;", left, op1, op2)
            }
            CasmInstruction::Mul { left, op1, op2 } => {
                write!(f, "{} = {} * {}, ap++;", left, op1, op2)
            }
            CasmInstruction::Deref { left, op } => {
                write!(f, "{} = {};", left, op)
            }
            CasmInstruction::Jmp(label) => {
                write!(f, "jmp {};", label)
            }
            CasmInstruction::JmpRel(offset) => {
                write!(f, "jmp rel {};", offset)
            }
            CasmInstruction::JmpIfNeq(offset, op) => {
                write!(f, "jmp rel {} if {} != 0", offset, op)
            }
            CasmInstruction::JmpIfNeqRel(offset, op) => {
                write!(f, "jmp rel {} if {} != 0", offset, op)
            }
        }
    }
}
