// Some notes and ideas:
//
// This file contains the "parser", which is not only a parser but both a parser and a bytecode compiler.
// It doesn't really compile to bytecode since what it compiles to not actualy byte-sized values,
// but the logic is basically the same.
//
// The plan was to generate an AST and then compile to "bytecode" but I didn't want to use Boxed values,
// and realized only now I could have used token indexes instead of referencing them or maybe move the
// values to a heap-like data structure since AST is a binary tree (probably more cache-efficient compared to indexes).
// Each statement would get its own tree.
//
// Anyway, the current architecture works for now, but eventually this will need to change if bc-r is to
// emulate POSIX bc correctly. Writing random numbers into the REPL pushes them to the stack, or just writing `+ +`
// results in generation of a `add` instruction but these should be a syntax error at some point.
// -- akgvn, 2021-03-22

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
    PushConstant(f64),
}

struct Parser<'source> {
    tokens: Vec<Token<'source>>,
    operations: Vec<Op<'source>>,
    current_idx: usize,
    had_error: bool,
}

impl<'source> Parser<'source> {
    pub fn new(tokens: Vec<Token<'source>>) -> Self {
        Self {
            tokens,
            operations: vec![],
            current_idx: 0,
            had_error: false,
        }
    }

    pub fn parse(&mut self) {
        loop {
            match self.get_current_token() {
                Token::EOF => return,
                Token::StatementEnd => {
                    self.advance();
                    self.parse_statement();
                }
                _ if self.current_idx > 0 => {
                    println!("{:?}", self.get_current_token());
                    panic!("This should _not_ happen.");
                }
                _ if self.had_error => return,
                _ => self.parse_statement(),
            };
        }
    }

    fn parse_statement(&mut self) {
        self.parse_expression();

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
    }

    fn parse_expression(&mut self) {
        let mut result_stack = vec![];
        let mut operator_stack = vec![];
        loop {
            // println!("Result stack right now: {:?}", result_stack);
            // println!("op stack right now: {:?}", operator_stack);
            if self.had_error {
                return;
            }

            let current_token = self.get_current_token();
            match current_token {
                Token::LeftParen => operator_stack.push(current_token),
                Token::RightParen => loop {
                    let popped_op = operator_stack.pop(); // for debugging
                    println!("{:?}", popped_op);
                    match popped_op {
                        Some(tok) => match tok {
                            Token::LeftParen => break,
                            _ => result_stack.push(tok),
                        },
                        None => {
                            self.error("No matching paranthesis.");
                            break;
                            // Error
                        }
                    }
                },
                Token::Plus | Token::Minus | Token::Star | Token::Slash => {
                    if self.is_unary_minus(self.current_idx) {
                        operator_stack.push(Token::UnaryMinus);
                    } else if let Some(last_op) = operator_stack.pop() {
                        if get_precedence(last_op) >= get_precedence(current_token) {
                            result_stack.push(last_op);
                            operator_stack.push(current_token);
                        } else {
                            operator_stack.push(last_op);
                            operator_stack.push(current_token);
                        }
                    } else {
                        operator_stack.push(current_token);
                    }
                }
                Token::Equals => {
                    // Note(ag): An identifier and an equals sign must be the first two
                    // tokens we encounter, we're not handling the error case when that
                    // is not true.
                    if let Some(last_tok) = result_stack.pop() {
                        match last_tok {
                            Token::Identifier(_, _) => {
                                operator_stack.push(last_tok);
                                operator_stack.push(current_token);
                            }
                            _ => {
                                // Syntax error, must be a ident just before an equals sign.
                                self.error("Syntax error, must assign to something.");
                            }
                        }
                    } else {
                        // Syntax error, must be a ident just before an equals sign.
                        self.error("Syntax error, must assign to something.");
                    }
                }
                Token::Number(_, _) | Token::Identifier(_, _) => result_stack.push(current_token),
                Token::UnaryMinus => panic!("Unary Minus in the wild."),
                Token::StatementEnd | Token::EOF => break,
            }
            self.advance();
        }

        // Make sure we did not leave anything in the operator stack.
        while let Some(op_tok) = operator_stack.pop() {
            result_stack.push(op_tok);
        }

        // Map result stack to operations.
        let mut assign_ident = false;
        for token in result_stack.into_iter() {
            match token {
                Token::Plus => self.operations.push(Op::Add),
                Token::Minus => self.operations.push(Op::Sub),
                Token::Star => self.operations.push(Op::Mult),
                Token::Slash => self.operations.push(Op::Div),
                Token::UnaryMinus => self.operations.push(Op::Negate),
                Token::Equals => {
                    assign_ident = true;
                }
                Token::Number(num, _) => {
                    let number: f64 = num.parse().unwrap();
                    self.operations.push(Op::PushConstant(number));
                }
                Token::Identifier(ident, _) => {
                    if assign_ident {
                        self.operations.push(Op::Assign(ident));
                        assign_ident = false;
                    } else {
                        self.operations.push(Op::GetVal(ident));
                    }
                }
                _ => self.error(""),
                /*
                panic!(
                    "Unreachable situation. Should not have pushed {:?} token into the results stack.",
                    token
                )
                */
            }
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

    // TODO This probably doesn't cover all cases.
    fn is_unary_minus(&self, minus_idx: usize) -> bool {
        if let Some(token) = self.tokens.get(minus_idx) {
            if *token != Token::Minus {
                return false;
            }
            if minus_idx == 0 {
                return true;
            }
            if let Some(prev) = self.tokens.get(minus_idx - 1) {
                match prev {
                    Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::Equals => {
                        return true
                    }
                    _ => {}
                }
            }
        }
        return false;
    }

    fn expect(&mut self, token: Token) {
        let current_token = self.get_current_token();
        if token != current_token {
            self.error("Didn't see expected token!");
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

    fn error(&mut self, msg: &str) {
        if !self.had_error {
            self.had_error = true;
            println!("Syntax error: {}", msg);
        }
    }
}

pub fn get_operations(tokens: Vec<Token>) -> (Vec<Op>, bool) {
    let mut parser = Parser::new(tokens);
    parser.parse();
    (parser.operations, parser.had_error)
}

fn get_precedence(tok: Token) -> u8 {
    match tok {
        Token::LeftParen => 3, // No one gets to pop paren except paren.
        Token::Plus => 25,
        Token::Minus => 25,
        Token::Star => 50,
        Token::Slash => 50,
        Token::Equals => 10,
        Token::UnaryMinus => 75,
        _ => 5,
    }
}
