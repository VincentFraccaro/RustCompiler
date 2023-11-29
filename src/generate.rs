use crate::parser::{NodeExpr, NodeProg, NodeStatement};
use std::collections::HashMap;
use std::fmt::Write;

/// `Generator` is responsible for generating assembly code from an AST (Abstract Syntax Tree).
pub struct Generator {
    program: NodeProg,
    stack_size: usize,
    vars: HashMap<String, Var>,
    stream: String,
}

/// Represents a variable with its stack location.
struct Var {
    stack_loc: usize,
}

impl Generator {
    /// Constructs a new code generator for a given program.
    pub fn new(program: NodeProg) -> Self {
        Self {
            program,
            stack_size: 0,
            vars: HashMap::new(),
            stream: String::new(),
        }
    }

    /// Generates assembly code for an expression.
    pub fn generate_expression(&mut self, expr: &NodeExpr) {
        match expr {
            NodeExpr::NodeExprIntLit(expr_int_lit) => {
                let value = expr_int_lit.int_lit.value.as_ref().expect("Expected int literal value");
                writeln!(&mut self.stream, "    mov rax, {}", value).expect("Failed to write to stream");
                self.push("rax");
            }
            NodeExpr::NodeExprIdent(expr_ident) => {
                let ident = expr_ident.ident.value.as_ref().expect("Expected identifier value");
                if let Some(var) = self.vars.get(ident) {
                    let offset = (self.stack_size - var.stack_loc - 1) * 8;
                    self.push(&format!("QWORD [rsp + {}]", offset));
                } else {
                    eprintln!("Undeclared identifier: {}", ident);
                    std::process::exit(1);
                }
            }
        }
    }

    /// Generates assembly code for a statement.
    pub fn generate_statement(&mut self, stmt: &NodeStatement) {
        match stmt {
            NodeStatement::NodeStatementExitEnum(stmt_exit) => {
                self.generate_expression(&stmt_exit.expr);
                writeln!(&mut self.stream, "    mov rax, 60").expect("Failed to write to stream");
                self.pop("rdi");
                writeln!(&mut self.stream, "    syscall").expect("Failed to write to stream");
            }
            NodeStatement::NodeStatementLetEnum(stmt_let) => {
                let ident = stmt_let.ident.value.as_ref().expect("Expected identifier value");
                if self.vars.contains_key(ident) {
                    eprintln!("Identifier already used: {}", ident);
                    std::process::exit(1);
                }
                self.vars.insert(
                    ident.clone(),
                    Var {
                        stack_loc: self.stack_size,
                    },
                );
                self.generate_expression(&stmt_let.expr);
            }
        }
    }

    /// Generates assembly code for the entire program.
    pub fn generate_program(&mut self) -> String {
        writeln!(&mut self.stream, "global _start\n_start:").expect("Failed to write to stream");

        let statements = self.program.statements.clone();
        for stmt in statements {
            self.generate_statement(&stmt);
        }

        writeln!(&mut self.stream, "    mov rax, 60").expect("Failed to write to stream");
        writeln!(&mut self.stream, "    mov rdi, 0").expect("Failed to write to stream");
        writeln!(&mut self.stream, "    syscall").expect("Failed to write to stream");
        self.stream.clone()
    }

    /// Helper function to push a register onto the stack.
    fn push(&mut self, reg: &str) {
        writeln!(&mut self.stream, "    push {}", reg).expect("Failed to write to stream");
        self.stack_size += 1;
    }

    /// Helper function to pop a register from the stack.
    fn pop(&mut self, reg: &str) {
        writeln!(&mut self.stream, "    pop {}", reg).expect("Failed to write to stream");
        self.stack_size -= 1;
    }
}
