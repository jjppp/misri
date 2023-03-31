use std::collections::HashMap;

use crate::{
    instr::{Operand, Program},
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Frame {
    map: HashMap<Operand, Value>,
    pub func: usize,
    pub pc: usize,
}

impl Frame {
    pub fn new(id: usize) -> Frame {
        Frame {
            map: HashMap::new(),
            pc: 0,
            func: id,
        }
    }

    pub fn get(&self, operand: &Operand) -> Option<&Value> {
        self.map.get(operand)
    }

    pub fn set(&mut self, operand: &Operand, value: &Value) {
        self.map.insert(operand.clone(), value.clone());
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
            stack: vec![Frame::new(program.entry)],
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
            Operand::Reg(name) => self
                .top_frame()
                .get(operand)
                .unwrap_or_else(|| panic!("{name} undefined"))
                .clone(),
        }
    }

    pub fn set(&mut self, operand: Operand, value: Value) {
        self.top_frame_mut().set(&operand, &value)
    }

    pub fn push_arg(&mut self, value: Value) {
        self.args.push(value)
    }

    pub fn pop_arg(&mut self) -> Value {
        self.args.pop().expect("arg stack empty")
    }

    pub fn push_frame(&mut self, id: usize) {
        self.stack.push(Frame::new(id))
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
            funcs: VecDeque::new(),
            entry: 0,
        });

        env.set(Operand::from("x"), Value::new_int(114));
        env.set(Operand::from("x"), Value::new_int(514));
        env.set(Operand::from("p"), Value::new_ptr(514));
        assert_eq!(env.get(&Operand::from("x")), Value::new_int(514));
        assert_eq!(env.get(&Operand::from("p")), Value::new_ptr(514));

        env.push_frame(Default::default());
        env.set(Operand::from("x"), Value::new_int(1919));
        assert_eq!(env.get(&Operand::from("x")), Value::new_int(1919));

        env.pop_frame();
        assert_eq!(env.get(&Operand::from("x")), Value::new_int(514));
        assert_eq!(env.get(&Operand::from("p")), Value::new_ptr(514));
    }
}
