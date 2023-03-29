use std::collections::LinkedList;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArithOp {
    OpAdd,
    OpSub,
    OpDiv,
    OpMul,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instr {
    IrAssign(Operand, Operand),
    IrArith(Operand, Operand, ArithOp, Operand),
    IrDeref(Operand, Operand),
    IrStore(Operand, Operand),
    IrLoad(Operand, Operand),
    IrLabel(String),
    IrGoto(String),
    IrCond(Operand, RelOp, Operand, String),
    IrReturn(Operand),
    IrDec(Operand, i32),
    IrArg(Operand),
    IrCall(Operand, String),
    IrParam(Operand),
    IrRead(Operand),
    IrWrite(Operand),
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub body: LinkedList<Instr>,
}

#[derive(Debug, Clone)]
pub struct Program {
    funcs: Vec<Func>,
}

impl Program {
    pub fn push(&mut self, func: Func) {
        self.funcs.push(func);
    }
}
