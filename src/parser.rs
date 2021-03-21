use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Negate,
    Mult,
    Div,
    Constant(f64),
}

struct Parser<'source> {
    tokens: Vec<Token<'source>>,
    operations: Vec<Op>,
    current_idx: usize,
}

impl<'source> Parser<'source> {
    pub fn new(tokens: Vec<Token<'source>>) -> Self {
        Self {
            tokens,
            operations: vec![],
            current_idx: 0,
        }
    }

    pub fn parse(&mut self) {
        /*loop {
            let token = self.tokens.get(self.current_idx);
            if token.is_none() {
                panic!("Shouldn't see none, should see EOF and get out.");
            }
            let token = token.unwrap();

            match token {
                Token::LeftParen => {}
                Token::RightParen => {}
                Token::Plus => {}
                Token::Minus => {}
                Token::Star => {}
                Token::Slash => {}
                Token::Number(src_str, line_num) => {}
                Token::Identifier(src_str, line_num) => {}
                Token::EOF => {}
            }
        }*/
        self.parse_expression();
    }

    fn parse_expression(&mut self) -> bool {
        match self.get_current_token() {
            Token::EOF => return false,
            _ => {}
        }
        self.parse_term();
        true
    }

    fn parse_term(&mut self) -> bool {
        let _ = self.parse_factor();

        loop {
            match self.get_current_token() {
                Token::Plus => {
                    self.advance();
                    self.parse_factor();
                    self.operations.push(Op::Add);
                }
                Token::Minus => {
                    self.advance();
                    self.parse_factor();
                    self.operations.push(Op::Sub);
                }
                Token::EOF => return false,
                _ => {
                    break;
                }
            }
        }
        false
    }

    fn parse_factor(&mut self) -> bool {
        let _ = self.parse_unary();

        loop {
            match self.get_current_token() {
                Token::Star => {
                    self.advance();
                    self.parse_unary();
                    self.operations.push(Op::Mult);
                }
                Token::Slash => {
                    self.advance();
                    self.parse_unary();
                    self.operations.push(Op::Div);
                }
                Token::EOF => return false,
                _ => {
                    break;
                }
            }
        }
        false
    }

    fn parse_unary(&mut self) -> bool {
        match self.get_current_token() {
            Token::Minus => {
                self.advance();
                self.parse_unary();
                self.operations.push(Op::Negate)
            }
            Token::Plus => {
                self.advance();
            }
            _ => {}
        }

        self.parse_grouping();
        false
    }

    fn parse_grouping(&mut self) -> bool {
        let _ = self.parse_literal();

        let token = self.get_current_token();
        match token {
            Token::LeftParen => {
                self.advance();
                self.parse_expression();
                self.expect(Token::RightParen);
            }
            _ => (),
        }
        false
    }

    fn parse_literal(&mut self) -> bool {
        let token = self.get_current_token();

        match token {
            Token::Number(num, _) => {
                let number: f64 = num.parse().unwrap();
                self.operations.push(Op::Constant(number));
                self.advance();
            }
            Token::EOF => return false,
            _ => (),
        }
        false
    }

    fn get_current_token(&self) -> Token {
        let token = self.tokens.get(self.current_idx);
        if token.is_none() {
            panic!("Shouldn't happen. EOF and out.");
        }
        *token.unwrap()
    }

    fn advance(&mut self) {
        self.current_idx += 1;
    }

    fn expect(&mut self, token: Token) {
        if token != self.get_current_token() {
            panic!("Didn't see expected!");
        } else {
            self.advance();
        }
    }
}

pub fn get_operations(tokens: Vec<Token>) -> Vec<Op> {
    let mut parser = Parser::new(tokens);
    parser.parse();
    parser.operations
}
