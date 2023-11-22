use crate::tokenisation::TokenType::{RETURN, SEMI, IntLit};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
    RETURN,
    SEMI,
    IntLit
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}

pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) value: Option<String>
}

pub fn tokenise(string: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut buf = String::new();
    let mut i = 0;

    while i < string.len() {
        if (string.as_bytes()[i] as char).is_alphabetic() {
            buf.push(string.as_bytes()[i] as char);
            println!("buf is {}", buf);
            i += 1;
            while (string.as_bytes()[i] as char).is_ascii_alphanumeric() {
                buf.push(string.as_bytes()[i] as char);
                println!("buf is {}", buf);
                i += 1;
            }
            if buf == "return" {
                tokens.push(Token { token_type: RETURN, value: None });
                buf.clear();
                println!("i is {} and buf is {}", i, buf);
            } else {
                panic!("You made boo boo");
            }
        }

        else if (string.as_bytes()[i] as char).is_digit(10) {
            buf.push(string.as_bytes()[i] as char);
            i += 1;
            while (string.as_bytes()[i] as char).is_digit(10) {
                buf.push(string.as_bytes()[i] as char);
                i += 1;
            }
            tokens.push(Token { token_type: IntLit, value: Option::from(buf.to_string()) });
            buf.clear();
            i -= 1;
        }

        else if (string.as_bytes()[i] as char) == ';' {
            tokens.push(Token{token_type: SEMI, value: None});
        }

        else if (string.as_bytes()[i] as char).is_whitespace(){

        }

        else {
            panic!("What did you do!?");
        }

        i += 1;
    }
    return tokens;
}