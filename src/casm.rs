use std::{fmt::{self, Debug, Display}};

#[derive(Clone)]
pub enum CasmInstruction {
    Ret,
    Call(String),
    IncrFp(i32),
    Label(String),
    Set{
        left : Operand,
        op : Operand,
    },
    Add{
        left : Operand,
        op1 : Operand,
        op2 : Operand,
    },
    Mul{
        left : Operand,
        op1 : Operand,
        op2 : Operand,
    },
    Deref{
        left : Operand,
        op : Operand,
    },
    JmpIfNeq(i32, Operand),
}

#[derive(Clone)]
pub enum Operand {
    Int(i32),
    DerefPc(i32),
    DerefFp(i32),
}

impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Int(n) => write!(f, "{}", n),
            Operand::DerefFp(offset) => write!(f, "[fp + {}]", offset),
            Operand::DerefPc(offset) => write!(f, "[pc + {}]", offset),
        }
    }
}

impl Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Int(n) => write!(f, "{}", n),
            Operand::DerefFp(offset) => write!(f, "[fp + {}]", offset),
            Operand::DerefPc(offset) => write!(f, "[pc + {}]", offset),
        }
    }
}

impl Debug for CasmInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CasmInstruction::Ret => write!(f, "jmp abs [fp - 1]"),
            CasmInstruction::Call(label) => write!(f, "jmp abs {}", label),
            CasmInstruction::IncrFp(n) => write!(f, "fp += {};", n),
            CasmInstruction::Label(label) => write!(f, "{}:", label),
            CasmInstruction::Set { left, op } => {
                write!(f, "{} = {};", left, op)
            }
            CasmInstruction::Add { left, op1, op2 } => {
                write!(f, "{} = {} + {};", left, op1, op2)
            }
            CasmInstruction::Mul { left, op1, op2 } => {
                write!(f, "{} = {} * {};", left, op1, op2)
            }
            CasmInstruction::Deref { left, op } => {
                write!(f, "{} = {};", left, op)
            }
            CasmInstruction::JmpIfNeq(offset, op) => {
                write!(f, "jmp rel {} if {} != 0", offset, op)
            }
        }
    }
}





