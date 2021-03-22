#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'source> {
    // The `usize`s used to be for line numbers, but I will not think about error handling just yet..
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Star,
    Slash,
    StatementEnd,
    Equals,
    Number(&'source str, usize),
    Identifier(&'source str, usize),
    EOF,
}

pub fn tokens_from_text(source: &str) -> Vec<Token> {
    let mut tk = Tokenizer::new(source);
    tk.tokenize();
    tk.tokens
}

pub struct Tokenizer<'source> {
    source_text: &'source str,
    chars: Vec<char>,
    current_idx: usize,
    line_num: usize,
    tokens: Vec<Token<'source>>,
}

impl<'source> Tokenizer<'source> {
    fn new(source_text: &'source str) -> Self {
        Self {
            source_text,
            chars: source_text.chars().collect(),
            current_idx: 0,
            line_num: 1,
            tokens: vec![],
        }
    }

    fn parse_number(&mut self) -> Token<'source> {
        let start = self.current_idx - 1;

        while let Some(ch) = self.chars.get(self.current_idx) {
            if is_digit(*ch) || *ch == '.' {
                self.current_idx += 1;
            } else {
                break;
            }
        }

        Token::Number(&self.source_text[start..self.current_idx], self.line_num)
    }

    fn parse_identifier(&mut self) -> Token<'source> {
        let start = self.current_idx - 1;

        while let Some(ch) = self.chars.get(self.current_idx) {
            if is_alphanumeric(*ch) {
                self.current_idx += 1;
            } else {
                break;
            }
        }

        Token::Identifier(&self.source_text[start..self.current_idx], self.line_num)
    }

    fn tokenize(&mut self) {
        loop {
            let ch = self.chars.get(self.current_idx);
            if ch.is_none() {
                break;
            }
            let ch = ch.unwrap();

            let token: Token;

            self.current_idx += 1;
            match ch {
                ' ' | '\t' | '\r' => {
                    continue;
                }
                '\n' => {
                    self.line_num += 1;
                    token = Token::StatementEnd;

                    if let Some(tok) = self.tokens.last() {
                        if *tok == Token::StatementEnd {
                            continue;
                        }
                    }
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    token = self.parse_identifier();
                }
                '0'..='9' => {
                    token = self.parse_number();
                }
                '+' => token = Token::Plus,
                '-' => token = Token::Minus,
                '*' => token = Token::Star,
                '/' => token = Token::Slash,
                '(' => token = Token::LeftParen,
                ')' => token = Token::RightParen,
                '=' => token = Token::Equals,
                _ => {
                    panic!("Weird char.");
                }
            }
            self.tokens.push(token);
        }
        self.tokens.push(Token::EOF);
    }
}

fn is_alphanumeric(ch: char) -> bool {
    match ch {
        '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false,
    }
}

fn is_digit(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        _ => false,
    }
}
