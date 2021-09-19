// Great resource for pratt parsers: https://www.oilshell.org/blog/2017/03/31.html

use crate::tokenizer::Token;
use std::fmt;

pub enum AstNode<'source> {
    Ident(&'source str),
    Number(f64),
    Op(Token<'source>, Vec<AstNode<'source>>),
}

impl<'source> fmt::Display for AstNode<'source> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Ident(ident) => write!(f, "{}", ident),
            AstNode::Number(number) => write!(f, "{}", number),
            AstNode::Op(op, children) => {
                write!(f, "({:?}", op)?;
                for node in children {
                    write!(f, " {}", node)?
                }
                write!(f, ")")
            }
        }
    }
}

pub struct Parser<'source> {
    tokens: Vec<Token<'source>>,
    current_idx: usize,
    had_error: bool,
}

impl<'source> Parser<'source> {
    pub fn new(tokens: Vec<Token<'source>>) -> Self {
        Self {
            tokens,
            current_idx: 0,
            had_error: false,
        }
    }

    pub fn parse(mut self) -> Vec<AstNode<'source>> {
        let mut statements = Vec::new();
        loop {
            match self.get_current_token() {
                Token::EOF => return statements,
                Token::StatementEnd => self.advance(),
                _ => statements.push(self.parse_expr(0)),
            }
        }
    }

    // TODO(ag): There are `self.advance()`s all over the place, so maybe
    // use .peek and .next like normal people. This can break very easily.
    fn parse_expr(&mut self, minimum_precedence: u8) -> AstNode<'source> {
        let tok = self.get_current_token();

        let mut left = match tok {
            Token::Number(number_str, _) => {
                self.advance();
                let number = number_str.parse().unwrap();
                AstNode::Number(number)
            }
            Token::Identifier(ident_str, _) => {
                self.advance();

                if self.get_current_token() == Token::LeftParen {
                    self.advance();
                    let args = self.parse_args();
                    self.advance();

                    // self.expect(Token::RightParen); TODO(ag) -> we skip over right paren somehow
                    AstNode::Op(Token::FnCall(ident_str), args)
                } else {
                    AstNode::Ident(ident_str)
                }
            }
            Token::LeftParen => {
                self.advance();
                let l = self.parse_expr(0);
                self.expect(Token::RightParen);
                self.advance();
                l
            }
            Token::Plus | Token::Minus => {
                self.advance();
                let right_prec = prefix_precedence(tok);
                let right = self.parse_expr(right_prec);
                AstNode::Op(tok, vec![right])
            }
            _ => {
                panic!("bad tok: {:?}", tok); // TODO proper error handling.
            }
        };

        loop {
            let tok = self.get_current_token();

            let op = match tok {
                Token::Plus | Token::Minus | Token::Star | Token::Power | Token::Slash | Token::Percent | Token::Equals => tok,
                Token::EOF | Token::StatementEnd => {
                    break;
                }
                Token::RightParen => {
                    break;
                }
                Token::ArgSeperator => {
                    break;
                }
                _ => {
                    panic!("bad op: {:?}", tok); // TODO proper error handling.
                }
            };
            let (left_prec, right_prec) = infix_precedence(op);
            if left_prec < minimum_precedence {
                break;
            }

            self.advance();
            let right = self.parse_expr(right_prec);

            left = AstNode::Op(op, vec![left, right]);
        }

        left
    }

    fn parse_args(&mut self) -> Vec<AstNode<'source>> /*???*/ {
        let mut args = Vec::new();
        while self.get_current_token() != Token::RightParen && self.get_current_token() != Token::ArgSeperator && self.get_current_token() != Token::StatementEnd {
            let arg = self.parse_expr(0);
            args.push(arg);
            self.advance();
        }
        args
    }

    fn get_current_token(&self) -> Token<'source> {
        *self.tokens.get(self.current_idx).unwrap_or(&Token::EOF)
    }

    fn advance(&mut self) {
        self.current_idx += 1;
    }

    fn expect(&mut self, token: Token) {
        let current_token = self.get_current_token();
        if token != current_token {
            self.error("Didn't see expected token!");
        }
    }

    fn error(&mut self, msg: &str) {
        if !self.had_error {
            self.had_error = true;
            println!("Syntax error: {}", msg);
        }
    }
}

fn prefix_precedence(tok: Token) -> u8 {
    match tok {
        Token::Plus | Token::Minus => 5,
        _ => {
            panic!("bad op: {:?}", tok); // TODO proper error handling.
        }
    }
}

fn infix_precedence(tok: Token) -> (u8, u8) {
    match tok {
        Token::Equals => (1, 0),
        Token::Percent => (1, 2),
        Token::Plus | Token::Minus => (3, 4),
        Token::Star | Token::Slash => (5, 6),
        Token::Power => (6, 7),
        _ => {
            panic!("bad op: {:?}", tok); // TODO proper error handling.
        }
    }
}
