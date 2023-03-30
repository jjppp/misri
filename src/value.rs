use std::{fmt::Display, ops};

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
    pub fn new_int(int: i32) -> Value {
        Value::ValInt(int)
    }

    pub fn new_ptr(size: usize) -> Value {
        Value::ValPtr {
            mem: Box::new(vec![0; size]),
            size,
            ptr: 0,
        }
    }

    pub fn load(&self) -> Value {
        match self {
            Value::ValPtr { mem, size, ptr } => {
                // TODO: bounds checking
                Value::ValInt(mem[ptr.to_owned()])
            }
            Value::ValInt(_) => panic!("cannot load ValInt"),
        }
    }

    pub fn store(&mut self, val: Value) {
        match self {
            Value::ValPtr { mem, size, ptr } => {
                // TODO: bounds checking
                if let Value::ValInt(int) = val {
                    mem[ptr.to_owned()] = int
                }
            }
            Value::ValInt(_) => panic!("cannot store ValInt!"),
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

impl ops::Sub<Value> for Value {
    type Output = Value;

    fn sub(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::ValInt(lhs), Value::ValInt(rhs)) => Value::ValInt(lhs - rhs),
            (Value::ValPtr { mem, size, ptr }, Value::ValInt(rhs)) => Value::ValPtr {
                mem,
                size,
                ptr: (ptr as i32 - rhs) as usize,
            },
            (Value::ValInt(lhs), Value::ValPtr { mem, size, ptr }) => Value::ValPtr {
                mem,
                size,
                ptr: (ptr as i32 - lhs) as usize,
            },
            _ => panic!("ptr - ptr"),
        }
    }
}

impl ops::Mul<Value> for Value {
    type Output = Value;

    fn mul(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::ValInt(lhs), Value::ValInt(rhs)) => Value::ValInt(lhs * rhs),
            (Value::ValPtr { mem, size, ptr }, Value::ValInt(rhs)) => Value::ValPtr {
                mem,
                size,
                ptr: (ptr as i32 * rhs) as usize,
            },
            (Value::ValInt(lhs), Value::ValPtr { mem, size, ptr }) => Value::ValPtr {
                mem,
                size,
                ptr: (ptr as i32 * lhs) as usize,
            },
            _ => panic!("ptr * ptr"),
        }
    }
}

impl ops::Div<Value> for Value {
    type Output = Value;

    fn div(self, rhs: Value) -> Value {
        match (self, rhs) {
            (Value::ValInt(lhs), Value::ValInt(rhs)) => Value::ValInt(lhs / rhs),
            (Value::ValPtr { mem, size, ptr }, Value::ValInt(rhs)) => Value::ValPtr {
                mem,
                size,
                ptr: (ptr as i32 / rhs) as usize,
            },
            (Value::ValInt(lhs), Value::ValPtr { mem, size, ptr }) => Value::ValPtr {
                mem,
                size,
                ptr: (ptr as i32 / lhs) as usize,
            },
            _ => panic!("ptr / ptr"),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::ValInt(lhs), Value::ValInt(rhs)) => lhs.partial_cmp(rhs),
            _ => None,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::new_int(0)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValInt(int) => writeln!(f, "{int}"),
            Self::ValPtr { .. } => {
                let value = self.load();
                writeln!(f, "{value}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
        let v1 = Value::new_int(114);
        let v2 = Value::new_int(514);
        assert_eq!(v1 + v2, Value::new_int(114 + 514))
    }

    #[test]
    fn test_ptr() {
        let mut p1 = Value::new_ptr(4);
        let offset = Value::new_int(2);

        p1.store(Value::ValInt(114));
        assert_eq!(p1.load(), Value::ValInt(114));

        let mut p2 = p1.clone() + offset;
        assert_eq!(p2.load(), Value::ValInt(0));

        p2.store(Value::ValInt(514));
        assert_eq!(p2.load(), Value::ValInt(514));
        assert_eq!(p1.load(), Value::ValInt(114))
    }
}
