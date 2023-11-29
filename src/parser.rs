use std::panic::panic_any;
use crate::parser::NodeStatement::{NodeStatementExitEnum, NodeStatementLetEnum};
use crate::tokenisation::TokenType::{ClosedParen, Equals, Identifier, IntLit, Let, OpenParen, RETURN, SEMI};
use crate::tokenisation::{Token, TokenType};

#[derive(Debug)]
pub enum NodeExpr {
    NodeExprIdent(NodeExprIdent),
    NodeExprIntLit(NodeExprIntLit),
}
#[derive(Debug)]
pub struct NodeExprIntLit {
    pub(crate) int_lit: Token
}
#[derive(Debug)]
pub struct NodeExprIdent {
    pub(crate) ident: Token
}

#[derive(Debug)]
pub enum NodeStatement {
    NodeStatementExitEnum(NodeStatementExit),
    NodeStatementLetEnum(NodeStatementLet),

}

#[derive(Debug)]
pub struct NodeStatementExit {
    pub(crate) expr: NodeExpr,
}
#[derive(Debug)]
pub struct NodeStatementLet {
    ident: Token,
    expr: NodeExpr,
}



pub struct NodeProg {
    pub(crate) statements: Vec<NodeStatement>,
}

pub struct Parser {
    pub(crate) m_tokens: Vec<Token>,
    m_index: usize,
}

impl Parser {
    pub fn new(m_tokens: Vec<Token>) -> Self {
        Self {
            m_tokens,
            m_index: 0,
        }
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        return self.m_tokens.get(self.m_index + offset);
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
        if self.peek(0).is_some() && self.peek(0).unwrap().token_type == IntLit {
            return Some(NodeExpr::NodeExprIntLit(
                NodeExprIntLit{ int_lit: self.consume().unwrap()
            }));
        }
        else if self.peek(0).is_some() && self.peek(0).unwrap().token_type == Identifier {
            return Some(NodeExpr::NodeExprIdent(
                NodeExprIdent{ ident: self.consume().unwrap()
                }));
        }
        else {
            println!("We are returning none");
            return None;
        }
    }

    pub fn parse_statement(&mut self) -> Option<NodeStatement> {
        if self.peek(0).unwrap().token_type == RETURN && self.peek(1).is_some() && self.peek(1).unwrap().token_type == OpenParen {
            self.consume();
            self.consume();
            println!("What token am I? {:?}", self.peek(0));
            let statement_exit: NodeStatementExit;
            if let Some(node_expr) = self.parse_expr() {
                statement_exit = NodeStatementExit{expr: node_expr};
                println!("I make it into self parse expr");
            } else {
                panic!("Invalid Expression in exit statement");
            }
            if self.peek(0).is_some() && self.peek(0).unwrap().token_type == ClosedParen {
                self.consume();
            }
            else {
                eprintln!("Expected a closed paren but didn't get");
            }
            if self.peek(0).is_some() && self.peek(0).unwrap().token_type == SEMI {
                self.consume();
            }
            else {
                eprintln!("This is bad");
            }
            return Some(NodeStatementExitEnum(statement_exit));


        }
        else if self.peek(0).is_some() && self.peek(0).unwrap().token_type == Let &&
                self.peek(1).is_some() && self.peek(1).unwrap().token_type == Identifier &&
                self.peek(2).is_some() && self.peek(2).unwrap().token_type == Equals
        {
            self.consume();
            let ident_token = self.consume().unwrap();
            let statement_let: NodeStatementLet;
            self.consume();
            let node_expr = self.parse_expr();
            if node_expr.is_some() {
                statement_let = NodeStatementLet{ ident: ident_token, expr: node_expr.unwrap()};
            }
            else {
                panic!("Had An issue making the NodeStatementLet");
            }
            if self.peek(0).is_some() && self.peek(0).unwrap().token_type == SEMI {
                self.consume();
            }
            else {
                panic!("Expected a semi");
            }
            return Some(NodeStatementLetEnum({statement_let}));
        }
        else {
            println!("I returned none?");
            return None;
        }
    }

    // Example function to demonstrate parsing logic
    pub fn parse_program(&mut self) -> Option<NodeProg> {
        let mut prog = NodeProg {
            statements: Vec::new(),
        };
        println!("I make it to the parse program");

        while self.peek(0).is_some() {
            println!("self peek is {:?}", self.peek(0));
            let stmt = self.parse_statement();
            if stmt.is_some() {
                prog.statements.push(stmt.unwrap());
                println!("I have made it in here");
            } else {
                panic!("Invalid in parse program");
            }
        }

        return Option::from(prog);
    }


}
