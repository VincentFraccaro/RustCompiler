use crate::tokenisation::TokenType::{ClosedParen, Equals, Identifier, IntLit, Let, OpenParen, RETURN, SEMI};
use std::option::Option;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    RETURN,
    IntLit,
    SEMI,
    OpenParen,
    ClosedParen,
    Identifier,
    Let,
    Equals,

}

#[derive(Debug, Clone)]
/// This struct takes a Token with a parameter of token_type and an optional value
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

pub struct Tokeniser {
    m_src: String,
    m_index: usize,
}

impl Tokeniser {
    pub fn new(m_src: String) -> Self {
        Self { m_src, m_index: 0 }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        if self.m_index + 1 > self.m_src.len() {
            return None;
        } else {
            self.m_src.chars().nth(self.m_index + offset)
        }
    }

    fn consume(&mut self) -> char {
        let ch = self
            .m_src
            .chars()
            .nth(self.m_index)
            .expect("No character to consume");
        self.m_index += 1;
        ch
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut buf = String::new();

        while let Some(ch) = self.peek(0) {
            match ch {
                c if c.is_alphabetic() => {
                    buf.push(self.consume());
                    while let Some(next_ch) = self.peek(0) {
                        if next_ch.is_alphanumeric() {
                            buf.push(self.consume());
                        } else {
                            break;
                        }
                    }

                    match buf.as_str() {
                        "return" => tokens.push(Token { token_type: RETURN, value: None }),
                        "let" => tokens.push(Token { token_type: Let, value: None}),
                        _ => tokens.push(Token {token_type: Identifier, value: Some(buf.clone())})
                    }
                    buf.clear();
                },
                c if c.is_digit(10) => {
                    buf.push(self.consume());
                    while let Some(next_ch) = self.peek(0) {
                        if next_ch.is_digit(10) {
                            buf.push(self.consume());
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token { token_type: IntLit, value: Some(buf.clone()) });
                    buf.clear();
                },
                ';' => {
                    self.consume();
                    tokens.push(Token { token_type: SEMI, value: None });
                },
                '=' => {
                    self.consume();
                    tokens.push(Token { token_type: Equals, value: None });
                },
                c if c.is_whitespace() => {
                    self.consume();
                },
                '(' => {
                    self.consume();
                    tokens.push(Token{token_type: OpenParen, value: None})
                },
                ')' => {
                    self.consume();
                    tokens.push(Token{token_type: ClosedParen, value: None})
                },
                _ => {
                    tokens.push(Token{token_type: Identifier, value: Some(buf.clone())});
                    buf.clear()
                }
            }
        }

        tokens
    }

}
