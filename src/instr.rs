use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Formatter},
};

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
    IrGoto {
        name: String,
        id: usize,
    },
    IrCond {
        x: Operand,
        op: RelOp,
        y: Operand,
        name: String,
        id: usize,
    },
    IrReturn(Operand),
    IrDec(Operand, i32),
    IrArg(Operand),
    IrCall {
        x: Operand,
        name: String,
        id: usize,
    },
    IrParam(Operand),
    IrRead(Operand),
    IrWrite(Operand),
}

impl Instr {
    pub fn new_goto(name: &str) -> Instr {
        Self::IrGoto {
            name: String::from(name),
            id: Default::default(),
        }
    }
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
            Self::IrGoto { name, .. } => write!(f, "GOTO {name} "),
            Self::IrCond { x, op, y, name, .. } => write!(f, "IF {x} {op} {y} GOTO {name}"),
            Self::IrReturn(x) => write!(f, "RETURN {x}"),
            Self::IrDec(x, size) => write!(f, "DEC {x} {size}"),
            Self::IrArg(x) => write!(f, "ARG {x}"),
            Self::IrCall { x, name, .. } => write!(f, "{x} := CALL {name}"),
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
    pub fn init(&mut self) {
        let mut map = HashMap::new();
        let mut id: usize = 0;
        for instr in &mut self.body {
            match instr {
                Instr::IrLabel(name) => {
                    id += 1;
                    map.insert(name.clone(), id);
                }
                Instr::IrGoto { name, .. } => {
                    let id = map.get(name).unwrap().clone();
                    *instr = Instr::IrGoto {
                        name: name.clone(),
                        id,
                    }
                }
                Instr::IrCond { x, op, y, name, .. } => {
                    let id = map.get(name).unwrap().clone();
                    *instr = Instr::IrCond {
                        x: x.clone(),
                        op: op.clone(),
                        y: y.clone(),
                        name: name.clone(),
                        id,
                    }
                }
                _ => (),
            }
        }
    }
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
    pub funcs: VecDeque<Func>,
    entry: usize,
}

impl Program {
    pub fn push_back(&mut self, func: Func) {
        self.funcs.push_back(func)
    }

    pub fn push_front(&mut self, func: Func) {
        self.funcs.push_front(func)
    }

    pub fn new() -> Program {
        Program {
            funcs: VecDeque::new(),
            entry: 0,
        }
    }

    pub fn fetch(&self, frame: &Frame) -> Instr {
        self.funcs[frame.func].body[frame.pc].clone()
    }

    pub fn init(&mut self) {
        self.funcs.iter_mut().for_each(|func| func.init());

        let mut map = HashMap::new();
        let mut id: usize = 0;
        for func in &self.funcs {
            id += 1;
            map.insert(func.name.clone(), id);
        }

        self.funcs
            .iter_mut()
            .flat_map(|func| func.body.iter_mut())
            .for_each(|instr| {
                if let Instr::IrCall { name, id, .. } = instr {
                    *id = map.get(name).unwrap().clone()
                }
            });
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for func in &self.funcs {
            writeln!(f, "{func}\n")?
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::*;

    #[test]
    fn test_init() {
        let mut parser = Parser::from(
            "FUNCTION foo :
         PARAM n
         i := #1
         s := #0
         LABEL loop :
         i := i + #1
         s := s + i
         IF i <= #100 GOTO loop
         RETURN s
         
         FUNCTION main :
         READ n
         ARG n
         s := CALL foo
         WRITE s
         RETURN #0",
        );

        let mut program = parser.parse();
        program.init();
        assert_eq!(
            program.funcs[0].body[6],
            Instr::IrCond {
                x: Operand::from("i"),
                op: RelOp::OpLE,
                y: Operand::from(100),
                name: String::from("loop"),
                id: 1
            }
        );
        assert_eq!(
            program.funcs[1].body[2],
            Instr::IrCall {
                x: Operand::from("s"),
                name: String::from("foo"),
                id: 1
            }
        )
    }
}
