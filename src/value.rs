use std::ops;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    ValInt(i32),
    ValPtr {
        mem: Box<Vec<i32>>,
        size: usize,
        ptr: usize,
    },
}

impl Value {
    pub fn newInt(int: i32) -> Value {
        Value::ValInt(int)
    }

    pub fn newPtr(size: usize) -> Value {
        Value::ValPtr {
            mem: Box::new(Vec::with_capacity(size)),
            size,
            ptr: 0,
        }
    }
}

impl ops::Add<Value> for Value {
    type Output = Value;

    fn add(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::ValInt(lhs), Value::ValInt(rhs)) => Value::ValInt(lhs + rhs),
            (Value::ValPtr { mem, size, ptr }, Value::ValInt(rhs)) => Value::ValPtr {
                mem,
                size,
                ptr: (ptr as i32 + rhs) as usize,
            },
            (Value::ValInt(lhs), Value::ValPtr { mem, size, ptr }) => Value::ValPtr {
                mem,
                size,
                ptr: (ptr as i32 + lhs) as usize,
            },
            _ => panic!("ptr + ptr"),
        }
    }
}
