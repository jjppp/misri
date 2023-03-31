use std::io;

use crate::{
    env::Env,
    instr::{ArithOp, Instr::*, Program, RelOp},
    value::Value,
};

pub fn exec(program: &Program) {
    let mut env = Env::new(program);
    loop {
        let instr = program.fetch(env.top_frame());
        match instr {
            Arith(x, y, op, z) => {
                env.pc_advance();
                let vy = env.get(&y);
                let vz = env.get(&z);
                let value = match op {
                    ArithOp::Add => vy + vz,
                    ArithOp::Sub => vy - vz,
                    ArithOp::Mul => vy * vz,
                    ArithOp::Div => vy / vz,
                };
                env.set(x, value)
            }
            Assign(x, y) => {
                env.pc_advance();
                env.set(x, env.get(&y))
            }
            Deref(x, y) => {
                env.pc_advance();
                env.set(x, env.get(&y))
            }
            Store(x, y) => {
                env.pc_advance();
                let val = env.get(&y);
                let addr = env.get(&x);
                addr.store(val);
            }
            Load(x, y) => {
                env.pc_advance();
                env.set(x, env.get(&y).load())
            }
            Arg(x) => {
                env.pc_advance();
                env.push_arg(env.get(&x))
            }
            Param(x) => {
                env.pc_advance();
                let value = env.pop_arg();
                env.set(x, value)
            }
            Label(_) => env.pc_advance(),
            Read(x) => {
                env.pc_advance();
                let buf = &mut String::new();
                io::stdin().read_line(buf).expect("input error");
                let int: i64 = buf.trim().parse().expect("input error");
                env.set(x, Value::new_int(int))
            }
            Write(x) => {
                env.pc_advance();
                let value = env.get(&x);
                println!("{value}")
            }
            Dec(x, size) => {
                env.pc_advance();
                env.set(x, Value::new_ptr(size as usize))
            }
            Call { id, .. } => {
                env.push_frame(id);
            }
            Return(x) => {
                if env.top_frame().func == program.entry {
                    return;
                }
                let value = env.get(&x);
                env.pop_frame();
                let func = &program.funcs[env.top_frame().func];
                match &func.body[env.pc()] {
                    Call { x, .. } => env.set(x.clone(), value),
                    _ => panic!("return error"),
                }
                env.pc_advance()
            }
            Goto { id, .. } => env.pc_set(id),
            Cond { x, op, y, id, .. } => {
                let vx = env.get(&x);
                let vy = env.get(&y);
                let jmp = match op {
                    RelOp::LT => vx < vy,
                    RelOp::LE => vx <= vy,
                    RelOp::GT => vx > vy,
                    RelOp::GE => vx >= vy,
                    RelOp::EQ => vx == vy,
                    RelOp::NE => vx != vy,
                };
                if jmp {
                    env.pc_set(id);
                } else {
                    env.pc_advance();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::*;

    #[test]
    fn test_exec() {
        let mut parser = Parser::from(
            "FUNCTION main :
             x := #114 * #514
             y := #0 - x
             WRITE y
             RETURN #0
             ",
        );
        let mut program = parser.parse();
        program.init();
        exec(&program);
    }

    #[test]
    fn test_arg() {
        let mut parser = Parser::from(
            "FUNCTION id :
             PARAM n
             RETURN n

             FUNCTION main :
             ARG #114
             x := CALL id
             ARG #514
             y := CALL id
             WRITE x
             WRITE y
             RETURN #0
             ",
        );
        let mut program = parser.parse();
        program.init();
        exec(&program);
    }

    #[test]
    fn test_fib() {
        let mut parser = Parser::from(
            " FUNCTION fib :
             PARAM n
             IF n != #0 GOTO br1
             RETURN #0
             LABEL br1 :
             IF n != #1 GOTO br2
             RETURN #1
             LABEL br2 :
             t1 := n - #1
             ARG t1
             r1 := CALL fib
             t2 := n - #2
             ARG t2
             r2 := CALL fib
             u := r1 + r2
             RETURN u
 
             FUNCTION main :
             READ n
             ARG n
             s := CALL fib
             WRITE s
             RETURN #0
            ",
        );
        let mut program = parser.parse();
        program.init();
        exec(&program);
    }

    #[test]
    fn test_arr() {
        let mut parser = Parser::from(
            "FUNCTION display :
             PARAM arr1
             c := *arr1
             WRITE c
             d := arr1 + #4
             c := *d
             WRITE c
             RETURN #0

             FUNCTION main :
             DEC tmp_1 24
             arr2 := &tmp_1
             *arr2 := #114
             tmp := arr2 + #4
             *tmp := #514
             ARG arr2
             uu  := CALL display
             RETURN #0
            ",
        );
        let mut program = parser.parse();
        program.init();
        exec(&program);
    }
}
