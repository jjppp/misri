use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Formatter},
};

use crate::env::Frame;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operand {
    Reg(String),
    Imm(i64),
}

impl From<i64> for Operand {
    fn from(int: i64) -> Operand {
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
    Add,
    Sub,
    Div,
    Mul,
}

impl Display for ArithOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelOp {
    LT,
    LE,
    GT,
    GE,
    EQ,
    NE,
}

impl Display for RelOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LT => write!(f, "<"),
            Self::LE => write!(f, "<="),
            Self::GT => write!(f, ">"),
            Self::GE => write!(f, ">="),
            Self::EQ => write!(f, "=="),
            Self::NE => write!(f, "!="),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instr {
    Assign(Operand, Operand),
    Arith(Operand, Operand, ArithOp, Operand),
    Deref(Operand, Operand),
    Store(Operand, Operand),
    Load(Operand, Operand),
    Label(String),
    Goto {
        name: String,
        id: usize,
    },
    Cond {
        x: Operand,
        op: RelOp,
        y: Operand,
        name: String,
        id: usize,
    },
    Return(Operand),
    Dec(Operand, i64),
    Arg(Operand),
    Call {
        x: Operand,
        name: String,
        id: usize,
    },
    Param(Operand),
    Read(Operand),
    Write(Operand),
}

impl Instr {
    pub fn new_goto(name: &str) -> Instr {
        Self::Goto {
            name: String::from(name),
            id: Default::default(),
        }
    }
}

impl Display for Instr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Assign(x, y) => write!(f, "{x} := {y}"),
            Self::Arith(x, y, op, z) => write!(f, "{x} := {y} {op} {z}"),
            Self::Deref(x, y) => write!(f, "{x} := &{y}"),
            Self::Store(x, y) => write!(f, "*{x} := {y}"),
            Self::Load(x, y) => write!(f, "{x} := *{y}"),
            Self::Label(name) => write!(f, "LABEL {name} :"),
            Self::Goto { name, .. } => write!(f, "GOTO {name} "),
            Self::Cond { x, op, y, name, .. } => write!(f, "IF {x} {op} {y} GOTO {name}"),
            Self::Return(x) => write!(f, "RETURN {x}"),
            Self::Dec(x, size) => write!(f, "DEC {x} {size}"),
            Self::Arg(x) => write!(f, "ARG {x}"),
            Self::Call { x, name, .. } => write!(f, "{x} := CALL {name}"),
            Self::Param(x) => write!(f, "PARAM {x}"),
            Self::Read(x) => write!(f, "READ {x}"),
            Self::Write(x) => write!(f, "WRITE {x}"),
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

        self.body.iter().enumerate().for_each(|(id, instr)| {
            if let Instr::Label(name) = instr {
                map.insert(name.clone(), id);
            }
        });

        for instr in &mut self.body {
            match instr {
                Instr::Goto { name, .. } => {
                    let id = *map.get(name).unwrap();
                    *instr = Instr::Goto {
                        name: name.clone(),
                        id,
                    }
                }
                Instr::Cond { id, name, .. } => {
                    *id = *map.get(name).unwrap_or_else(|| panic!("{name}"));
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
                Instr::Label(_) => writeln!(f, "{instr}")?,
                _ => writeln!(f, "  {instr}")?,
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Program {
    pub funcs: VecDeque<Func>,
    pub entry: usize,
}

impl Program {
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

        let mut map: HashMap<String, usize> = HashMap::new();
        for (id, func) in self.funcs.iter().enumerate() {
            map.insert(func.name.clone(), id);
        }

        self.entry = *map
            .get(&String::from("main"))
            .expect("no main function found");

        self.funcs
            .iter_mut()
            .flat_map(|func| func.body.iter_mut())
            .for_each(|instr| {
                if let Instr::Call { name, id, .. } = instr {
                    *id = *map.get(name).unwrap()
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
            Instr::Cond {
                x: Operand::from("i"),
                op: RelOp::LE,
                y: Operand::from(100),
                name: String::from("loop"),
                id: 3
            }
        );
        assert_eq!(
            program.funcs[1].body[2],
            Instr::Call {
                x: Operand::from("s"),
                name: String::from("foo"),
                id: 0
            }
        );
        assert_eq!(program.entry, 1);
    }
}
