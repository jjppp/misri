use std::io;

use crate::{
    env::Env,
    instr::{ArithOp, Instr::*, Program, RelOp},
    value::Value,
};

pub fn exec(program: Program) {
    let mut env = Env::new();
    loop {
        let instr = program.fetch(env.top_frame());
        match instr {
            IrArith(x, y, op, z) => {
                let vy = env.get(y);
                let vz = env.get(z);
                let value = match op {
                    ArithOp::OpAdd => vy + vz,
                    ArithOp::OpSub => vy - vz,
                    ArithOp::OpMul => vy * vz,
                    ArithOp::OpDiv => vy / vz,
                };
                env.set(x, value)
            }
            IrAssign(x, y) => {
                env.pc_advance();
                env.set(x, env.get(y))
            }
            IrDeref(x, y) => {
                env.pc_advance();
                env.set(x, env.get(y))
            }
            IrStore(x, y) => {
                env.pc_advance();
                env.get(x).store(env.get(y))
            }
            IrLoad(x, y) => {
                env.pc_advance();
                env.set(x, env.get(y).load())
            }
            IrArg(x) => {
                env.pc_advance();
                env.push_arg(env.get(x))
            }
            IrParam(x) => {
                env.pc_advance();
                let value = env.pop_arg();
                env.set(x, value)
            }
            IrLabel(_) => env.pc_advance(),
            IrRead(x) => {
                env.pc_advance();
                let buf = &mut String::new();
                io::stdin().read_line(buf).expect("input error");
                let int: i32 = buf.trim().parse().expect("input error");
                env.set(x, Value::new_int(int))
            }
            IrWrite(x) => {
                env.pc_advance();
                let value = env.get(x);
                println!("{value}")
            }
            IrDec(x, size) => {
                env.pc_advance();
                env.set(x, Value::new_ptr(size as usize))
            }
            IrCall { id, .. } => {
                env.push_frame(id);
            }
            IrReturn(x) => {
                let value = env.get(x);
                env.pop_frame();
                let func = &program.funcs[env.top_frame().func];
                match &func.body[env.pc()] {
                    IrCall { x, .. } => env.set(x.clone(), value),
                    _ => panic!("return error"),
                }
                env.pc_advance()
            }
            IrGoto { id, .. } => env.pc_set(id),
            IrCond { x, op, y, id, .. } => {
                let vx = env.get(x);
                let vy = env.get(y);
                let jmp = match op {
                    RelOp::OpLT => vx < vy,
                    RelOp::OpLE => vx <= vy,
                    RelOp::OpGT => vx > vy,
                    RelOp::OpGE => vx >= vy,
                    RelOp::OpEQ => vx == vy,
                    RelOp::OpNE => vx != vy,
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
