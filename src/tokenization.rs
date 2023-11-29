use crate::tokenization::TokenType::*;
use std::iter::Peekable;
use std::str::Chars;

/// An enum representing the different types of tokens that can be identified.
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

/// Represents a single token in the tokenization process.
///
/// Each token has a type and, optionally, an associated value.
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

/// A tokenizer for parsing a string into a series of tokens.
///
/// The tokenizer uses a peekable iterator over the characters of the input string,
/// allowing it to look ahead without consuming characters immediately.
pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    /// Creates a new tokenizer for the given input string.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice reference to the input text to be tokenized.
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    /// Peeks at the next character in the input string without consuming it.
    ///
    /// Returns an option containing a reference to the next character,
    /// or None if the end of the string is reached.
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    /// Consumes the next character in the input string.
    ///
    /// Returns the character if available, or None if the end of the string is reached.
    fn consume(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Tokenizes the entire input string.
    ///
    /// Processes the input string character by character and groups them into tokens
    /// based on the rules defined for alphabetic, digit, and special characters.
    ///
    /// Returns a vector of tokens parsed from the input string.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut buf = String::new();

        while let Some(&ch) = self.peek() {
            match ch {
                c if c.is_alphabetic() => {
                    if let Some(c) = self.consume() {
                        buf.push(c);
                        while let Some(&next_ch) = self.peek() {
                            if next_ch.is_alphanumeric() {
                                if let Some(next_c) = self.consume() {
                                    buf.push(next_c);
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    let token_type = match buf.as_str() {
                        "return" => RETURN,
                        "let" => Let,
                        _ => Identifier,
                    };

                    tokens.push(Token {
                        token_type,
                        value: if token_type == Identifier {
                            Some(buf.clone())
                        } else {
                            None
                        },
                    });
                    buf.clear();
                }
                c if c.is_digit(10) => {
                    if let Some(c) = self.consume() {
                        buf.push(c);
                        while let Some(&next_ch) = self.peek() {
                            if next_ch.is_digit(10) {
                                if let Some(next_c) = self.consume() {
                                    buf.push(next_c);
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }

                    tokens.push(Token {
                        token_type: IntLit,
                        value: Some(buf.clone()),
                    });
                    buf.clear();
                }
                ';' => {
                    self.consume();
                    tokens.push(Token {
                        token_type: SEMI,
                        value: None,
                    });
                }
                '=' => {
                    self.consume();
                    tokens.push(Token {
                        token_type: Equals,
                        value: None,
                    });
                }
                c if c.is_whitespace() => {
                    self.consume();
                }
                '(' => {
                    self.consume();
                    tokens.push(Token {
                        token_type: OpenParen,
                        value: None,
                    });
                }
                ')' => {
                    self.consume();
                    tokens.push(Token {
                        token_type: ClosedParen,
                        value: None,
                    });
                }
                _ => {
                    // Handle unexpected characters
                    self.consume();
                }
            }
        }

        tokens
    }
}
