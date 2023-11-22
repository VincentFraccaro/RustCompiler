use std::fmt;
use crate::tokenisation::Token;
use crate::tokenisation::TokenType::{IntLit, RETURN, SEMI};

pub fn tokens_to_assembly(tokens: Vec<Token>) -> String {
    let mut stream = String::new();
    fmt::Write::write_str(&mut stream, "global _start\n_start:\n").unwrap();

    let mut i = 0;
    while i < tokens.len() {
        let token: &Token = &tokens[i];

        if token.token_type == RETURN {
            if (i + 1 < tokens.len()) && (tokens[i + 1].token_type == IntLit) {
                if (i + 2 < tokens.len()) && (tokens[i + 2].token_type == SEMI) {
                    fmt::Write::write_str(&mut stream, "    mov rax, 60\n").unwrap();

                    // Handle the Option<String> for token value
                    let token_value = match &tokens[i + 1].value {
                        Some(value) => value.as_str(),
                        None => "",
                    };

                    fmt::Write::write_str(&mut stream, &format!("    mov rdi, {}\n", token_value)).unwrap();
                    fmt::Write::write_str(&mut stream, "    syscall").unwrap();
                }
            }
        }
        i += 1;
    }
    return stream
}