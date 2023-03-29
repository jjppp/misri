use std::collections::HashMap;

use crate::{instr::Operand, value::Value};

#[derive(Debug, Clone)]
pub struct Env {
    map: HashMap<Operand, Value>,
}

impl Env {
    pub fn get(&self, operand: &Operand) -> Value {
        self.map.get(operand).unwrap().clone()
    }
    pub fn set(&mut self, operand: &Operand, value: &Value) -> () {
        self.map.insert(operand.clone(), value.clone());
    }
}
