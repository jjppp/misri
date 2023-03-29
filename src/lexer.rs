#[derive(Debug)]
pub struct Lexer {
    char_stream: CharStream,
    curr: Token,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    TokIden(String),
    TokInt(i32),
    TokFunc,
    TokLabel,
    TokIf,
    TokGoto,
    TokReturn,
    TokDec,
    TokArg,
    TokCall,
    TokParam,
    TokRead,
    TokWrite,
    TokColon,
    TokLT,
    TokLE,
    TokGT,
    TokGE,
    TokEQ,
    TokNE,
    TokSharp,
    TokAssign,
    TokAdd,
    TokSub,
    TokStar,
    TokDiv,
    TokAmp,
    TokEOF,
}

impl Lexer {
    pub fn from(input: String) -> Lexer {
        let mut lexer = Lexer {
            char_stream: CharStream::from(input),
            curr: Token::TokEOF,
        };
        lexer.consume();
        lexer
    }

    pub fn consume(&mut self) -> Token {
        let result = self.peek();
        self.curr = match self.char_stream.peek() {
            None => Token::TokEOF,
            Some(' ' | '\t' | '\n' | '\r') => {
                self.char_stream.consume();
                return self.consume();
            }
            Some('0'..='9') => self.lex_int(),
            Some('a'..='z' | 'A'..='Z' | '_') => self.lex_iden(),
            Some('#') => {
                self.char_stream.consume();
                Token::TokSharp
            }
            Some('+') => {
                self.char_stream.consume();
                Token::TokAdd
            }
            Some('-') => {
                self.char_stream.consume();
                Token::TokSub
            }
            Some('*') => {
                self.char_stream.consume();
                Token::TokStar
            }
            Some('/') => {
                self.char_stream.consume();
                Token::TokDiv
            }
            Some('=') => {
                self.char_stream.consume();
                match self.char_stream.peek() {
                    Some('=') => {
                        self.char_stream.consume();
                        Token::TokEQ
                    }
                    ch => panic!("lex error: {:?}", ch),
                }
            }
            Some('<') => {
                self.char_stream.consume();
                match self.char_stream.peek() {
                    Some('=') => {
                        self.char_stream.consume();
                        Token::TokLE
                    }
                    _ => Token::TokLT,
                }
            }
            Some('>') => {
                self.char_stream.consume();
                match self.char_stream.peek() {
                    Some('=') => {
                        self.char_stream.consume();
                        Token::TokGE
                    }
                    _ => Token::TokGT,
                }
            }
            Some(':') => {
                self.char_stream.consume();
                match self.char_stream.peek() {
                    Some('=') => {
                        self.char_stream.consume();
                        Token::TokAssign
                    }
                    _ => Token::TokColon,
                }
            }
            Some('&') => {
                self.char_stream.consume();
                Token::TokAmp
            }
            ch => panic!("lex error: {:?}", ch),
        };
        result
    }

    pub fn peek(&mut self) -> Token {
        self.curr.clone()
    }

    fn lex_int(&mut self) -> Token {
        let mut int: i32 = 0;
        loop {
            let ch = self.char_stream.peek();
            match ch {
                Some('0'..='9') => int = int * 10 + ch.and_then(|x| x.to_digit(10)).unwrap() as i32,
                None | Some(_) => return Token::TokInt(int),
            }
            self.char_stream.consume();
        }
    }

    fn lex_iden(&mut self) -> Token {
        let mut iden = String::new();
        loop {
            let ch = self.char_stream.peek();
            match ch {
                Some(ch) => {
                    if ch.is_ascii_alphanumeric() || ch == '_' {
                        iden.push(self.char_stream.consume())
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        match iden.as_str() {
            "FUNCTION" => Token::TokFunc,
            "LABEL" => Token::TokLabel,
            "IF" => Token::TokIf,
            "GOTO" => Token::TokGoto,
            "RETURN" => Token::TokReturn,
            "DEC" => Token::TokDec,
            "ARG" => Token::TokArg,
            "CALL" => Token::TokCall,
            "PARAM" => Token::TokParam,
            "READ" => Token::TokRead,
            "WRITE" => Token::TokWrite,
            _ => Token::TokIden(iden),
        }
    }
}

#[derive(Debug)]
pub struct CharStream {
    input: String,
    curr: usize,
}

impl CharStream {
    pub fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.curr)
    }

    pub fn consume(&mut self) -> char {
        let ch = self.peek().unwrap();
        self.curr += 1;
        ch
    }

    pub fn from(input: String) -> CharStream {
        CharStream { input, curr: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int() {
        let mut lexer = Lexer::from(String::from("114 514 1919 810"));
        assert_eq!(lexer.peek(), Token::TokInt(114));
        assert_eq!(lexer.peek(), Token::TokInt(114));

        assert_eq!(lexer.consume(), Token::TokInt(114));
        assert_eq!(lexer.consume(), Token::TokInt(514));
        assert_eq!(lexer.consume(), Token::TokInt(1919));
        assert_eq!(lexer.consume(), Token::TokInt(810));

        assert_eq!(lexer.peek(), Token::TokEOF);
        assert_eq!(lexer.peek(), Token::TokEOF);
        assert_eq!(lexer.consume(), Token::TokEOF);
        assert_eq!(lexer.consume(), Token::TokEOF);
    }

    #[test]
    fn test_iden() {
        let mut lexer = Lexer::from(String::from("x y z a_1 __b23"));
        assert_eq!(lexer.peek(), Token::TokIden(String::from("x")));
        assert_eq!(lexer.peek(), Token::TokIden(String::from("x")));

        assert_eq!(lexer.consume(), Token::TokIden(String::from("x")));
        assert_eq!(lexer.consume(), Token::TokIden(String::from("y")));
        assert_eq!(lexer.consume(), Token::TokIden(String::from("z")));
        assert_eq!(lexer.consume(), Token::TokIden(String::from("a_1")));
        assert_eq!(lexer.consume(), Token::TokIden(String::from("__b23")));

        assert_eq!(lexer.peek(), Token::TokEOF);
        assert_eq!(lexer.peek(), Token::TokEOF);
        assert_eq!(lexer.consume(), Token::TokEOF);
        assert_eq!(lexer.consume(), Token::TokEOF);
    }
}
