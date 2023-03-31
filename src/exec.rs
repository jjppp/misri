use std::{
    fmt::Debug,
    io::{BufRead, BufReader, BufWriter, Write},
};

use crate::{
    env::Env,
    instr::{ArithOp, Instr::*, Program, RelOp},
    value::Value,
};

pub struct Interpreter<T, U>
where
    U: std::io::Write,
{
    program: Program,
    env: Env,
    fin: BufReader<T>,
    fout: BufWriter<U>,
}

impl<T, U> Interpreter<T, U>
where
    T: std::io::Read,
    U: std::io::Write + Debug,
{
    pub fn new(mut program: Program, fin: T, fout: U) -> Self
    where
        T: std::io::Read,
        U: std::io::Write,
    {
        program.init();
        let env = Env::new(&program);
        Interpreter {
            program,
            env,
            fin: BufReader::new(fin),
            fout: BufWriter::new(fout),
        }
    }

    pub fn exec(&mut self) -> usize {
        let mut instr_cnt = 0;
        while let Some(next_pc) = self.step() {
            self.env.pc_set(next_pc);
            instr_cnt += 1
        }
        instr_cnt
    }

    pub fn step(&mut self) -> Option<usize> {
        let program = &self.program;
        let env = &mut self.env;
        let instr = program.fetch(env.top_frame());
        match instr {
            Arith(x, y, op, z) => {
                let vy = env.get(&y);
                let vz = env.get(&z);
                let value = match op {
                    ArithOp::Add => vy + vz,
                    ArithOp::Sub => vy - vz,
                    ArithOp::Mul => vy * vz,
                    ArithOp::Div => vy / vz,
                };
                env.set(x, value);
                Some(env.pc_next())
            }
            Assign(x, y) => {
                env.set(x, env.get(&y));
                Some(env.pc_next())
            }
            Deref(x, y) => {
                env.set(x, env.get(&y));
                Some(env.pc_next())
            }
            Store(x, y) => {
                let val = env.get(&y);
                let addr = env.get(&x);
                addr.store(val);
                Some(env.pc_next())
            }
            Load(x, y) => {
                env.set(x, env.get(&y).load());
                Some(env.pc_next())
            }
            Arg(x) => {
                env.push_arg(env.get(&x));
                Some(env.pc_next())
            }
            Param(x) => {
                let value = env.pop_arg();
                env.set(x, value);
                Some(env.pc_next())
            }
            Label(_) => Some(env.pc_next()),
            Read(x) => {
                let buf = &mut String::new();
                self.fin.read_line(buf).expect("input error");
                let int: i64 = buf.trim().parse().expect("input error");
                env.set(x, Value::new_int(int));
                Some(env.pc_next())
            }
            Write(x) => {
                let value = env.get(&x);
                writeln!(self.fout, "{value}").expect("write error");
                Some(env.pc_next())
            }
            Dec(x, size) => {
                env.set(x, Value::new_ptr(size as usize));
                Some(env.pc_next())
            }
            Call { id, .. } => {
                env.push_frame(id);
                Some(env.pc())
            }
            Return(x) => {
                if env.top_frame().func == program.entry {
                    return None;
                }
                let value = env.get(&x);
                env.pop_frame();
                let func = &program.funcs[env.top_frame().func];
                match &func.body[env.pc()] {
                    Call { x, .. } => env.set(x.clone(), value),
                    _ => panic!("return error"),
                };
                Some(env.pc_next())
            }
            Goto { id, .. } => Some(id),
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
                    Some(id)
                } else {
                    Some(env.pc_next())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    use super::*;

    fn config(code: &str, input: &str, output: &str) {
        let mut parser = Parser::from(code);
        let program = parser.parse();
        let mut interpreter = Interpreter::new(program, input.as_bytes(), Vec::new());
        interpreter.exec();

        assert_eq!(interpreter.fout.into_inner().unwrap(), output.as_bytes());
    }

    #[test]
    fn test_exec() {
        config(
            "FUNCTION main :
             x := #114 * #514
             y := #0 - x
             WRITE y
             RETURN #0
             ",
            "",
            "-58596\n",
        );
    }

    #[test]
    fn test_arg() {
        config(
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
            "",
            "114\n514\n",
        );
    }

    #[test]
    fn test_fib() {
        config(
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
            "10\n",
            "55\n",
        )
    }

    #[test]
    fn test_arr() {
        config(
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
            "",
            "114\n\
            514\n",
        );
    }
}
