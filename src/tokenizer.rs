#[derive(Debug)]
pub enum Token<'source> {
    Plus(usize),
    Minus(usize),
    Star(usize),
    Slash(usize),
    Number(&'source str, usize),
    Identifier(&'source str, usize),
    EOF,
}

pub struct Tokenizer<'source> {
    source_text: &'source str,
    chars: Vec<char>,
    current_idx: usize,
    line_num: usize,
    tokens: Vec<Token<'source>>,
}

impl<'source> Tokenizer<'source> {
    pub fn new(source_text: &'source str) -> Self {
        Self {
            source_text,
            chars: source_text.chars().collect(),
            current_idx: 0,
            line_num: 1,
            tokens: vec![],
        }
    }

    fn parse_number(&mut self) -> Token<'source> {
        let start = self.current_idx;

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
        let start = self.current_idx;

        while let Some(ch) = self.chars.get(self.current_idx) {
            if is_alphanumeric(*ch) {
                self.current_idx += 1;
            } else {
                break;
            }
        }

        Token::Identifier(&self.source_text[start..self.current_idx], self.line_num)
    }

    pub fn tokenize(&mut self) {
        loop {
            let ch = self.chars.get(self.current_idx);
            if ch.is_none() {
                break;
            }
            let ch = ch.unwrap();

            let token: Token;

            match ch {
                ' ' | '\t' | '\r' => {
                    self.current_idx += 1;
                    continue;
                }
                '\n' => {
                    self.current_idx += 1;
                    self.line_num += 1;
                    continue;
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    token = self.parse_identifier();
                }
                '0'..='9' => {
                    token = self.parse_number();
                }
                '+' => {
                    token = Token::Plus(self.line_num);
                    self.current_idx += 1;
                }
                '-' => {
                    token = Token::Minus(self.line_num);
                    self.current_idx += 1;
                }
                '*' => {
                    token = Token::Star(self.line_num);
                    self.current_idx += 1;
                }
                '/' => {
                    token = Token::Slash(self.line_num);
                    self.current_idx += 1;
                }
                _ => {
                    panic!("Weird char.");
                }
            }
            self.tokens.push(token);
        }
        self.tokens.push(Token::EOF);
    }

    pub fn get_tokens(self) -> Vec<Token<'source>> {
        self.tokens
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
