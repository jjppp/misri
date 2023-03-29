use std::collections::LinkedList;

use crate::{
    instr::{ArithOp, Func, Instr, Operand, Program, RelOp},
    lexer::{Lexer, Token},
};

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    body: LinkedList<Instr>,
}

impl Parser {
    pub fn from(input: &str) -> Parser {
        Parser {
            lexer: Lexer::from(String::from(input)),
            body: LinkedList::new(),
        }
    }

    pub fn parse(&mut self) -> Program {
        let token = self.lexer.peek();
        match token {
            Token::TokFunc => {
                let fun = self.parse_func();
                let mut program = self.parse();
                program.push(fun);
                program
            }
            Token::TokEOF => Program { funcs: Vec::new() },
            token => panic!("parse error: {:?}", token),
        }
    }

    fn parse_func(&mut self) -> Func {
        self.lexer.consume();
        let name = self.parse_name();
        self.lexer.consume();
        self.body = LinkedList::new();
        self.parse_body();
        let func = Func {
            name,
            body: self.body.clone(),
        };
        func
    }

    fn parse_instr(&mut self) -> Instr {
        match self.lexer.peek() {
            Token::TokLabel => {
                self.lexer.consume();
                let name = self.parse_name();
                self.lexer.consume();
                Instr::IrLabel(name)
            }
            Token::TokIden(_) => {
                let x = self.parse_operand();
                self.lexer.consume();
                match self.lexer.peek() {
                    Token::TokAmp => {
                        self.lexer.consume();
                        let y = self.parse_operand();
                        Instr::IrDeref(x, y)
                    }
                    Token::TokStar => {
                        self.lexer.consume();
                        let y = self.parse_operand();
                        Instr::IrLoad(x, y)
                    }
                    Token::TokCall => {
                        self.lexer.consume();
                        let f_name = self.parse_name();
                        Instr::IrCall(x, f_name)
                    }
                    Token::TokIden(_) | Token::TokSharp => {
                        let y = self.parse_operand();
                        match self.lexer.peek() {
                            Token::TokAdd | Token::TokSub | Token::TokStar | Token::TokDiv => {
                                let op = self.parse_arith_op();
                                let z = self.parse_operand();
                                Instr::IrArith(x, y, op, z)
                            }
                            _ => Instr::IrAssign(x, y),
                        }
                    }
                    token => panic!("parse error: {:?}", token),
                }
            }
            Token::TokStar => {
                self.lexer.consume();
                let lhs = self.parse_operand();
                self.lexer.consume();
                let rhs = self.parse_operand();
                Instr::IrStore(lhs, rhs)
            }
            Token::TokGoto => {
                self.lexer.consume();
                Instr::IrGoto(self.parse_name())
            }
            Token::TokIf => {
                self.lexer.consume();
                let lhs = self.parse_operand();
                let op = self.parse_rel_op();
                let rhs = self.parse_operand();
                self.lexer.consume();
                let name = self.parse_name();
                Instr::IrCond(lhs, op, rhs, name)
            }
            Token::TokReturn => {
                self.lexer.consume();
                Instr::IrReturn(self.parse_operand())
            }
            Token::TokDec => {
                self.lexer.consume();
                let tar = self.parse_operand();
                let size = self.parse_int();
                Instr::IrDec(tar, size)
            }
            Token::TokArg => {
                self.lexer.consume();
                Instr::IrArg(self.parse_operand())
            }
            Token::TokParam => {
                self.lexer.consume();
                Instr::IrParam(self.parse_operand())
            }
            Token::TokRead => {
                self.lexer.consume();
                Instr::IrRead(self.parse_operand())
            }
            Token::TokWrite => {
                self.lexer.consume();
                Instr::IrWrite(self.parse_operand())
            }
            token => panic!("parse error: {:?}", token),
        }
    }

    fn parse_operand(&mut self) -> Operand {
        match self.lexer.consume() {
            Token::TokSharp => Operand::Imm(self.parse_int()),
            Token::TokIden(name) => Operand::Reg(name),
            token => panic!("parse error: {:?}", token),
        }
    }

    fn parse_int(&mut self) -> i32 {
        match self.lexer.consume() {
            Token::TokInt(int) => int,
            token => panic!("parse error: {:?}", token),
        }
    }

    fn parse_rel_op(&mut self) -> RelOp {
        match self.lexer.consume() {
            Token::TokLT => RelOp::OpLT,
            Token::TokLE => RelOp::OpLE,
            Token::TokGT => RelOp::OpGT,
            Token::TokGE => RelOp::OpGE,
            Token::TokEQ => RelOp::OpEQ,
            Token::TokNE => RelOp::OpNE,
            token => panic!("parse error: {:?}", token),
        }
    }

    fn parse_arith_op(&mut self) -> ArithOp {
        match self.lexer.consume() {
            Token::TokAdd => ArithOp::OpAdd,
            Token::TokSub => ArithOp::OpSub,
            Token::TokStar => ArithOp::OpMul,
            Token::TokDiv => ArithOp::OpDiv,
            token => panic!("parse error: {:?}", token),
        }
    }

    fn parse_name(&mut self) -> String {
        match self.lexer.consume() {
            Token::TokIden(name) => name,
            token => panic!("parse error: {:?}", token),
        }
    }

    fn parse_body(&mut self) {
        match self.lexer.peek() {
            Token::TokFunc | Token::TokEOF => (),
            Token::TokIf
            | Token::TokLabel
            | Token::TokIden(_)
            | Token::TokStar
            | Token::TokGoto
            | Token::TokReturn
            | Token::TokWrite
            | Token::TokRead
            | Token::TokParam
            | Token::TokDec
            | Token::TokArg => {
                let instr = self.parse_instr();
                self.body.push_back(instr);
                self.parse_body()
            }
            token => panic!("parse error: {:?}", token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instr() {
        let mut parser = Parser::from(
            "x := y
             x := y + z
             x := y - z
             x := y * z
             x := y / z
             x := &y
             x := *y
             *x := y
             GOTO wjp
             LABEL wjp :
             IF x < y GOTO wjp
             RETURN x
             DEC arr 24
             ARG x
             y := CALL foo
             PARAM x
             READ x
             WRITE x",
        );
        assert_eq!(
            parser.parse_instr(),
            Instr::IrAssign(Operand::from("x"), Operand::from("y"))
        );
        assert_eq!(
            parser.parse_instr(),
            Instr::IrArith(
                Operand::from("x"),
                Operand::from("y"),
                ArithOp::OpAdd,
                Operand::from("z")
            )
        );
        assert_eq!(
            parser.parse_instr(),
            Instr::IrArith(
                Operand::from("x"),
                Operand::from("y"),
                ArithOp::OpSub,
                Operand::from("z")
            )
        );
        assert_eq!(
            parser.parse_instr(),
            Instr::IrArith(
                Operand::from("x"),
                Operand::from("y"),
                ArithOp::OpMul,
                Operand::from("z")
            )
        );
        assert_eq!(
            parser.parse_instr(),
            Instr::IrArith(
                Operand::from("x"),
                Operand::from("y"),
                ArithOp::OpDiv,
                Operand::from("z")
            )
        );
        assert_eq!(
            parser.parse_instr(),
            Instr::IrDeref(Operand::from("x"), Operand::from("y"))
        );
        assert_eq!(
            parser.parse_instr(),
            Instr::IrLoad(Operand::from("x"), Operand::from("y"))
        );
        assert_eq!(
            parser.parse_instr(),
            Instr::IrStore(Operand::from("x"), Operand::from("y"))
        );
        assert_eq!(parser.parse_instr(), Instr::IrGoto(String::from("wjp")));
        assert_eq!(parser.parse_instr(), Instr::IrLabel(String::from("wjp")));
        assert_eq!(
            parser.parse_instr(),
            Instr::IrCond(
                Operand::from("x"),
                RelOp::OpLT,
                Operand::from("y"),
                String::from("wjp")
            )
        );
        assert_eq!(parser.parse_instr(), Instr::IrReturn(Operand::from("x")));
        assert_eq!(parser.parse_instr(), Instr::IrDec(Operand::from("arr"), 24));
        assert_eq!(parser.parse_instr(), Instr::IrArg(Operand::from("x")));
        assert_eq!(
            parser.parse_instr(),
            Instr::IrCall(Operand::from("y"), String::from("foo"))
        );
        assert_eq!(parser.parse_instr(), Instr::IrParam(Operand::from("x")));
        assert_eq!(parser.parse_instr(), Instr::IrRead(Operand::from("x")));
        assert_eq!(parser.parse_instr(), Instr::IrWrite(Operand::from("x")));
    }

    #[test]
    fn test_func() {
        let mut parser = Parser::from(
            "FUNCTION fact :
             PARAM v1
             IF v1 == #1 GOTO label1
             GOTO label2
             LABEL label1 :
             RETURN v1
             LABEL label2 :
             t1 := v1 - #1
             ARG t1
             t2 := CALL fact
             t3 := v1 * t2
             RETURN t3",
        );
        let func = parser.parse_func();
        assert_eq!(func.name, String::from("fact"));
        assert_eq!(func.body.len(), 11);
        assert_eq!(
            func.body,
            LinkedList::from([
                Instr::IrParam(Operand::from("v1")),
                Instr::IrCond(
                    Operand::from("v1"),
                    RelOp::OpEQ,
                    Operand::from(1),
                    String::from("label1")
                ),
                Instr::IrGoto(String::from("label2")),
                Instr::IrLabel(String::from("label1")),
                Instr::IrReturn(Operand::from("v1")),
                Instr::IrLabel(String::from("label2")),
                Instr::IrArith(
                    Operand::from("t1"),
                    Operand::from("v1"),
                    ArithOp::OpSub,
                    Operand::from(1)
                ),
                Instr::IrArg(Operand::from("t1")),
                Instr::IrCall(Operand::from("t2"), String::from("fact")),
                Instr::IrArith(
                    Operand::from("t3"),
                    Operand::from("v1"),
                    ArithOp::OpMul,
                    Operand::from("t2")
                ),
                Instr::IrReturn(Operand::from("t3")),
            ])
        )
    }

    #[test]
    fn test_program() {
        let mut parser = Parser::from(
            "FUNCTION add :
             PARAM v1
             t2 := *v1
             t7 := v1 + #4
             t3 := *t7
             t1 := t2 + t3
             RETURN t1

             FUNCTION main :
             DEC v3 8
             t9 := &v3
             *t9 := #1
             t12 := t10 + #4
             *t12 := #2
             ARG t10
             t14 := CALL add
             v2 := t14
             WRITE v2
             RETURN #0",
        );
        let program = parser.parse();
        assert_eq!(program.funcs.len(), 2);
    }
}
