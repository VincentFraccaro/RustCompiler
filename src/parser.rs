use crate::tokenisation::TokenType::{IntLit, SEMI};
use crate::tokenisation::{Token, TokenType};

#[derive(Debug)]
pub struct NodeExpr {
    pub(crate) int_lit: Token,
}

#[derive(Debug)]
pub struct NodeExit {
    pub(crate) expr: NodeExpr,
}

pub struct Parser {
    m_tokens: Vec<Token>,
    m_index: usize,
}

impl Parser {
    pub fn new(m_tokens: Vec<Token>) -> Self {
        Self {
            m_tokens,
            m_index: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        return self.m_tokens.get(self.m_index);
    }

    fn consume(&mut self) -> Option<Token> {
        if self.m_index < self.m_tokens.len() {
            let token = self.m_tokens[self.m_index].clone();
            self.m_index += 1;
            Some(token)
        } else {
            None
        }
    }

    pub fn parse_expr(&mut self) -> Option<NodeExpr> {
        if self.peek().is_some() && self.peek().unwrap().token_type == IntLit {
            return Some(NodeExpr {
                int_lit: (self.consume().unwrap()),
            });
        } else {
            println!("We are returning none");
            return None;
        }
    }

    // Example function to demonstrate parsing logic
    pub fn parse(&mut self) -> Option<NodeExit> {
        let mut exit_node: Option<NodeExit> = None;
        while let Some(token) = self.peek() {
            match token.token_type {
                TokenType::RETURN => {
                    self.consume(); // Consume RETURN token
                    let node_expr = self.parse_expr();
                    if node_expr.is_some() {
                        exit_node = Some(NodeExit {
                            expr: node_expr.unwrap(),
                        });
                        println!("We are making an exit node");
                    } else {
                        panic!("Invalid Expression");
                    }
                    println!("What am i {:?}", self.peek().unwrap());
                    if self.peek().is_some() && self.peek().unwrap().token_type == SEMI {
                        self.consume();
                        println!("We are consuming the semi");
                    }
                    println!("Consumed RETURN");
                }
                _ => {}
            }
        }
        self.m_index = 0;
        println!("exit node is {:?}", exit_node.as_mut().unwrap().expr);
        if exit_node.is_some() {
            return exit_node;
        } else {
            return None;
        }
    }
}
