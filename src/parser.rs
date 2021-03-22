use crate::tokenizer::Token;

#[derive(Debug)]
pub enum Op<'source> {
    Add,
    Sub,
    Negate,
    Mult,
    Div,
    GetVal(&'source str),
    Assign(&'source str),
    Constant(f64),
}

struct Parser<'source> {
    tokens: Vec<Token<'source>>,
    operations: Vec<Op<'source>>,
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
        self.parse_statement();
    }

    fn parse_statement(&mut self) {
        loop {
            match self.get_current_token() {
                Token::Equals => {
                    if let Some(val_str) = self.get_prev_ident() {
                        self.advance();
                        self.parse_expression();
                        self.operations.push(Op::Assign(val_str));
                    } else {
                    }
                }
                Token::StatementEnd | Token::EOF => return,
                _ => {}
            }
            self.parse_expression();
        }
    }

    fn parse_expression(&mut self) {
        match self.get_current_token() {
            Token::EOF => return,
            _ => {}
        }
        self.parse_term();
    }

    fn parse_term(&mut self) {
        self.parse_factor();

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
                Token::EOF => return,
                _ => {
                    break;
                }
            }
        }
    }

    fn parse_factor(&mut self) {
        self.parse_unary();

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
                Token::EOF => return,
                _ => {
                    break;
                }
            }
        }
    }

    fn parse_unary(&mut self) {
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
    }

    fn parse_grouping(&mut self) {
        self.parse_literal();

        let token = self.get_current_token();
        match token {
            Token::LeftParen => {
                self.advance();
                self.parse_expression();
                self.expect(Token::RightParen);
            }
            _ => (),
        }
    }

    fn parse_literal(&mut self) {
        let token = self.get_current_token();

        match token {
            Token::Number(num, _) => {
                let number: f64 = num.parse().unwrap();
                self.operations.push(Op::Constant(number));
                self.advance();
            }
            Token::Identifier(val_str, _) => {
                self.operations.push(Op::GetVal(val_str));
                self.advance();
            }
            Token::EOF => return,
            _ => (),
        }
    }

    fn get_current_token(&self) -> Token<'source> {
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

    fn get_prev_ident(&mut self) -> Option<&'source str> {
        let last = self.operations.pop();
        if last.is_none() {
            return None;
        }
        let last = last.unwrap();
        match last {
            Op::GetVal(val_str) => Some(val_str),
            _ => {
                self.operations.push(last);
                None
            }
        }
    }
}

pub fn get_operations(tokens: Vec<Token>) -> Vec<Op> {
    let mut parser = Parser::new(tokens);
    parser.parse();
    parser.operations
}
