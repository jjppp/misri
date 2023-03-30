use std::fmt::{Display, Formatter};

use crate::env::Frame;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operand {
    Reg(String),
    Imm(i32),
}

impl From<i32> for Operand {
    fn from(int: i32) -> Operand {
        Operand::Imm(int)
    }
}

impl From<&str> for Operand {
    fn from(name: &str) -> Operand {
        Operand::Reg(String::from(name))
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reg(name) => write!(f, "{name}"),
            Self::Imm(int) => write!(f, "{int}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithOp {
    OpAdd,
    OpSub,
    OpDiv,
    OpMul,
}

impl Display for ArithOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpAdd => write!(f, "+"),
            Self::OpSub => write!(f, "-"),
            Self::OpMul => write!(f, "*"),
            Self::OpDiv => write!(f, "/"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelOp {
    OpLT,
    OpLE,
    OpGT,
    OpGE,
    OpEQ,
    OpNE,
}

impl Display for RelOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpLT => write!(f, "<"),
            Self::OpLE => write!(f, "<="),
            Self::OpGT => write!(f, ">"),
            Self::OpGE => write!(f, ">="),
            Self::OpEQ => write!(f, "=="),
            Self::OpNE => write!(f, "!="),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instr {
    IrAssign(Operand, Operand),
    IrArith(Operand, Operand, ArithOp, Operand),
    IrDeref(Operand, Operand),
    IrStore(Operand, Operand),
    IrLoad(Operand, Operand),
    IrLabel(String),
    IrGoto(String, usize),
    IrCond(Operand, RelOp, Operand, String, usize),
    IrReturn(Operand),
    IrDec(Operand, i32),
    IrArg(Operand),
    IrCall(Operand, String, usize),
    IrParam(Operand),
    IrRead(Operand),
    IrWrite(Operand),
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IrAssign(x, y) => write!(f, "{x} := {y}"),
            Self::IrArith(x, y, op, z) => write!(f, "{x} := {y} {op} {z}"),
            Self::IrDeref(x, y) => write!(f, "{x} := &{y}"),
            Self::IrStore(x, y) => write!(f, "*{x} := {y}"),
            Self::IrLoad(x, y) => write!(f, "{x} := *{y}"),
            Self::IrLabel(name) => write!(f, "LABEL {name} :"),
            Self::IrGoto(name, _) => write!(f, "GOTO {name} "),
            Self::IrCond(x, op, y, name, _) => write!(f, "IF {x} {op} {y} GOTO {name}"),
            Self::IrReturn(x) => write!(f, "RETURN {x}"),
            Self::IrDec(x, size) => write!(f, "DEC {x} {size}"),
            Self::IrArg(x) => write!(f, "ARG {x}"),
            Self::IrCall(x, name, _) => write!(f, "{x} := CALL {name}"),
            Self::IrParam(x) => write!(f, "PARAM {x}"),
            Self::IrRead(x) => write!(f, "READ {x}"),
            Self::IrWrite(x) => write!(f, "WRITE {x}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub body: Vec<Instr>,
}

impl Func {
    pub fn init(&mut self) {}
}

impl Display for Func {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        writeln!(f, "FUNCTION {name} :")?;
        for instr in &self.body {
            match instr {
                Instr::IrLabel(_) => writeln!(f, "{instr}")?,
                _ => writeln!(f, "  {instr}")?,
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub funcs: Vec<Func>,
    entry: usize,
}

impl Program {
    pub fn push(&mut self, func: Func) {
        self.funcs.push(func)
    }

    pub fn new() -> Program {
        Program {
            funcs: Vec::new(),
            entry: 0,
        }
    }

    pub fn fetch(&self, frame: &Frame) -> Instr {
        self.funcs[frame.func].body[frame.pc].clone()
    }

    pub fn init(&mut self) {}
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for func in &self.funcs {
            writeln!(f, "{func}\n")?
        }
        Ok(())
    }
}
