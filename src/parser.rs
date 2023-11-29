use crate::tokenization::Token;
use crate::tokenization::TokenType::{ClosedParen, Equals, Identifier, IntLit, Let, OpenParen, RETURN, SEMI};

/// Represents an expression node in the abstract syntax tree.
#[derive(Clone, Debug)]
pub enum NodeExpr {
    NodeExprIdent(NodeExprIdent),
    NodeExprIntLit(NodeExprIntLit),
}

/// Integer literal expression node.
#[derive(Clone, Debug)]
pub struct NodeExprIntLit {
    pub(crate) int_lit: Token,
}

/// Identifier expression node.
#[derive(Clone, Debug)]
pub struct NodeExprIdent {
    pub(crate) ident: Token,
}

/// Represents a statement node in the abstract syntax tree.
#[derive(Clone, Debug)]
pub enum NodeStatement {
    NodeStatementExitEnum(NodeStatementExit),
    NodeStatementLetEnum(NodeStatementLet),
}

/// Exit statement node.
#[derive(Clone, Debug)]
pub struct NodeStatementExit {
    pub(crate) expr: NodeExpr,
}

/// Let statement node.
#[derive(Clone, Debug)]
pub struct NodeStatementLet {
    pub(crate) ident: Token,
    pub(crate) expr: NodeExpr,
}

/// Represents a program node, consisting of a series of statements.
pub struct NodeProg {
    pub(crate) statements: Vec<NodeStatement>,
}

/// Parser for converting tokens into abstract syntax tree nodes.
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    /// Constructs a new parser from a list of tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    /// Peeks at the token at the specified offset from the current index.
    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.index + offset)
    }

    /// Consumes and returns the current token, advancing the parser index.
    fn consume(&mut self) -> Option<Token> {
        self.tokens.get(self.index).cloned().map(|token| {
            self.index += 1;
            token
        })
    }

    /// Parses an expression from the token stream.
    pub fn parse_expr(&mut self) -> Result<NodeExpr, String> {
        let next_token = self.peek(0).ok_or("Unexpected end of input")?;
        match next_token.token_type {
            IntLit => Ok(NodeExpr::NodeExprIntLit(NodeExprIntLit {
                int_lit: self.consume().unwrap(),
            })),
            Identifier => Ok(NodeExpr::NodeExprIdent(NodeExprIdent {
                ident: self.consume().unwrap(),
            })),
            _ => Err("Unexpected token type in expression".to_string()),
        }
    }

    /// Parses a statement from the token stream.
    pub fn parse_statement(&mut self) -> Result<NodeStatement, String> {
        let next_token = self.peek(0).ok_or("Unexpected end of input")?;

        match next_token.token_type {
            RETURN if self.peek(1).map_or(false, |t| t.token_type == OpenParen) => {
                self.consume(); // Consume 'Return'
                self.consume(); // Consume '('

                let expr = self.parse_expr()?;
                if self.peek(0).map_or(false, |t| t.token_type == ClosedParen) {
                    self.consume(); // Consume ')'
                } else {
                    return Err("Expected closing parenthesis".to_string());
                }
                if self.peek(0).map_or(false, |t| t.token_type == SEMI) {
                    self.consume(); // Consume ';'
                } else {
                    return Err("Expected semicolon".to_string());
                }

                Ok(NodeStatement::NodeStatementExitEnum(NodeStatementExit { expr }))
            },
            Let if self.peek(1).map_or(false, |t| t.token_type == Identifier) => {
                self.consume(); // Consume 'Let'
                let ident_token = self.consume().ok_or("Expected identifier")?;
                if self.peek(0).map_or(false, |t| t.token_type == Equals) {
                    self.consume(); // Consume '='
                } else {
                    return Err("Expected equals sign".to_string());
                }

                let expr = self.parse_expr()?;
                if self.peek(0).map_or(false, |t| t.token_type == SEMI) {
                    self.consume(); // Consume ';'
                } else {
                    return Err("Expected semicolon".to_string());
                }

                Ok(NodeStatement::NodeStatementLetEnum(NodeStatementLet { ident: ident_token, expr }))
            },
            _ => Err("Unexpected token type in statement".to_string()),
        }
    }

    pub fn parse_program(&mut self) -> Result<NodeProg, String> {
        let mut statements = Vec::new();

        while self.peek(0).is_some() {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
        }

        Ok(NodeProg { statements })
    }
}

// Continue refactoring other methods
