// Great resource of pratt parsers: https://www.oilshell.org/blog/2017/03/31.html

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

    pub fn parse(mut self) -> AstNode<'source> {
        self.parse_expr(0)
    }

    // TODO(ag): There are `self.advance()`s all over the place, so maybe
    // use .peek and .next like normal people. This can break very easily.
    fn parse_expr(&mut self, minimum_precedence: u8) -> AstNode<'source> {
        let tok = self.get_current_token();
        println!(
            "START --- id: {:?}, token: {:?} --- minprec: {}",
            self.current_idx, tok, minimum_precedence
        );
        let mut left = match tok {
            Token::Number(number_str, _) => {
                self.advance();
                let number = number_str.parse().unwrap();
                AstNode::Number(number)
            }
            Token::Identifier(ident_str, _) => {
                self.advance();
                AstNode::Ident(ident_str)
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
                panic!("bad tok: {:?}", tok);
            }
        };

        loop {
            let tok = self.get_current_token();
            println!(
                "LOOP  --- id: {:?}, token: {:?} --- minprec: {}",
                self.current_idx, tok, minimum_precedence
            );
            let op = match tok {
                Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Equals => tok,
                Token::EOF | Token::StatementEnd => {
                    break;
                }
                Token::RightParen => {
                    break;
                }
                _ => {
                    panic!("bad op: {:?}", tok);
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
        println!("out");
        left
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
            panic!("bad op: {:?}", tok);
        }
    }
}

fn infix_precedence(tok: Token) -> (u8, u8) {
    match tok {
        Token::Equals => (1, 0),
        Token::Plus | Token::Minus => (1, 2),
        Token::Star | Token::Slash => (3, 4),
        _ => {
            panic!("bad op: {:?}", tok);
        }
    }
}
