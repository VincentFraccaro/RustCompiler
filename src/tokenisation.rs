use crate::tokenisation::TokenType::{IntLit, RETURN, SEMI};
use std::option::Option;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    RETURN,
    IntLit,
    SEMI,
}

#[derive(Debug, Clone)]
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

    fn peek(&self) -> Option<char> {
        if self.m_index + 1 > self.m_src.len() {
            return None;
        } else {
            self.m_src.chars().nth(self.m_index)
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

        while self.peek().is_some() {
            if self.peek().unwrap().is_alphabetic() {
                buf.push(self.consume());
                while self.peek().is_some() && self.peek().unwrap().is_alphanumeric() {
                    buf.push(self.consume());
                }
                println!("{}", buf);
                if buf == "return" {
                    tokens.push(Token {
                        token_type: RETURN,
                        value: None,
                    });
                    buf.clear();
                } else {
                    panic!("We got an issue with trying to make a return token");
                }
            } else if self.peek().unwrap().is_digit(10) {
                buf.push(self.consume());
                while self.peek().is_some() && self.peek().unwrap().is_digit(10) {
                    buf.push(self.consume());
                }
                tokens.push(Token {
                    token_type: IntLit,
                    value: Option::from(buf.to_string()),
                });
                buf.clear();
            } else if self.peek() == Option::from(';') {
                self.consume();
                tokens.push(Token {
                    token_type: SEMI,
                    value: None,
                });
            } else if self.peek().unwrap().is_whitespace() {
                self.consume();
            } else {
                panic!("Something weird has happened");
            }
        }

        return tokens;
    }
}
