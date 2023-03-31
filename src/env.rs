use crate::{
    instr::{Func, Operand, Program},
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Frame {
    map: Vec<Value>,
    pub func: usize,
    pub pc: usize,
}

impl Frame {
    pub fn new(func: &Func) -> Frame {
        Frame {
            map: vec![Value::default(); func.nreg + 1],
            pc: 0,
            func: func.id,
        }
    }

    pub fn get(&self, id: &usize) -> Option<&Value> {
        self.map.get(*id)
    }

    pub fn set(&mut self, id: &usize, value: &Value) {
        self.map[*id] = value.clone();
    }
}

#[derive(Debug, Clone)]
pub struct Env {
    stack: Vec<Frame>,
    args: Vec<Value>,
}

impl Env {
    pub fn new(program: &Program) -> Env {
        Env {
            stack: vec![Frame::new(&program.funcs[program.entry])],
            args: Vec::new(),
        }
    }

    pub fn top_frame_mut(&mut self) -> &mut Frame {
        self.stack.last_mut().unwrap()
    }

    pub fn top_frame(&self) -> &Frame {
        self.stack.last().unwrap()
    }

    pub fn pc_next(&self) -> usize {
        self.top_frame().pc + 1
    }

    pub fn pc_set(&mut self, pc: usize) {
        self.top_frame_mut().pc = pc
    }

    pub fn pc(&self) -> usize {
        self.top_frame().pc
    }

    pub fn get(&self, operand: &Operand) -> Value {
        match operand {
            Operand::Imm(int) => Value::new_int(*int),
            Operand::Reg { name, id } => self
                .top_frame()
                .get(id)
                .unwrap_or_else(|| panic!("{name} undefined"))
                .clone(),
        }
    }

    pub fn set(&mut self, operand: Operand, value: Value) {
        if let Operand::Reg { id, .. } = operand {
            self.top_frame_mut().set(&id, &value)
        }
    }

    pub fn push_arg(&mut self, value: Value) {
        self.args.push(value)
    }

    pub fn pop_arg(&mut self) -> Value {
        self.args.pop().expect("arg stack empty")
    }

    pub fn push_frame(&mut self, func: &Func) {
        self.stack.push(Frame::new(func))
    }

    pub fn pop_frame(&mut self) {
        self.stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[test]
    fn test_get_set() {
        let mut env = Env::new(&Program {
            funcs: VecDeque::from([Func {
                name: String::from("foo"),
                body: Vec::new(),
                nreg: 2,
                id: 0,
            }]),
            entry: 0,
        });

        env.set(Operand::from(("x", 0)), Value::new_int(114));
        env.set(Operand::from(("x", 0)), Value::new_int(514));
        env.set(Operand::from(("p", 1)), Value::new_ptr(514));
        assert_eq!(env.get(&Operand::from(("x", 0))), Value::new_int(514));
        assert_eq!(env.get(&Operand::from(("p", 1))), Value::new_ptr(514));

        env.push_frame(&Func {
            name: String::new(),
            body: Vec::new(),
            nreg: 2,
            id: 0,
        });
        env.set(Operand::from(("x", 0)), Value::new_int(1919));
        assert_eq!(env.get(&Operand::from(("x", 0))), Value::new_int(1919));

        env.pop_frame();
        assert_eq!(env.get(&Operand::from(("x", 0))), Value::new_int(514));
        assert_eq!(env.get(&Operand::from(("p", 1))), Value::new_ptr(514));
    }
}
