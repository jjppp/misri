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
    pub fn from(input: String) -> Parser {
        Parser {
            lexer: Lexer::from(input),
            body: LinkedList::new(),
        }
    }

    pub fn parse(&mut self) -> Program {
        let tok = self.lexer.consume();
        match tok {
            Token::TokFunc => {
                let fun = self.parse_func();
                let mut program = self.parse();
                program.push(fun);
                program
            }
            _ => panic!("parse error"),
        }
    }

    fn parse_func(&mut self) -> Func {
        self.body = LinkedList::new();
        self.parse_body();
        let func = Func {
            name: self.parse_name(),
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
                    _ => panic!("parse error"),
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
            _ => panic!("parse error"),
        }
    }

    fn parse_operand(&mut self) -> Operand {
        match self.lexer.consume() {
            Token::TokSharp => Operand::Imm(self.parse_int()),
            Token::TokIden(name) => Operand::Reg(name),
            _ => panic!("parse error"),
        }
    }

    fn parse_int(&mut self) -> i32 {
        match self.lexer.consume() {
            Token::TokInt(int) => int,
            _ => panic!("parse error"),
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
            _ => panic!("parse error"),
        }
    }

    fn parse_arith_op(&mut self) -> ArithOp {
        match self.lexer.consume() {
            Token::TokAdd => ArithOp::OpAdd,
            Token::TokSub => ArithOp::OpSub,
            Token::TokStar => ArithOp::OpMul,
            Token::TokDiv => ArithOp::OpDiv,
            _ => panic!("parse error"),
        }
    }

    fn parse_name(&mut self) -> String {
        match self.lexer.consume() {
            Token::TokIden(name) => name,
            _ => panic!("parse error"),
        }
    }

    fn parse_body(&mut self) {
        match self.lexer.peek() {
            Token::TokFunc => (),
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
                self.body.push_back(instr)
            }
            _ => panic!("parse error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instr() {
        let mut parser = Parser::from(String::from(
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
        ));
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
}
