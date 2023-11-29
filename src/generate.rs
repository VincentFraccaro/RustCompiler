use crate::parser::{NodeExpr, NodeProg, NodeStatement};

use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;
use std::fmt::Write;

pub struct Generator {
    m_program: NodeProg,
    m_stack_size: usize,
    stream: String
}

impl Generator {
    
    pub fn new(m_program: NodeProg) -> Self {
        Self { m_program, m_stack_size: 0 , stream: "".to_string() }
    }

    pub fn pop(&mut self, reg: &str) {
        write!(&mut self.stream, "    mov rax, {}\n", reg).unwrap();
        self.m_stack_size -= 1;
    }

    pub fn generate_expression(&mut self, expr: &NodeExpr) -> String{
        println!("I am generating expressions");
        match expr {
            NodeExpr::NodeExprIntLit(expr_int_lit) => {
                fmt::Write::write_str(&mut self.stream, concat!("{}{}{}", "    mov rax, ", 1, "\n")).unwrap();
            },
            NodeExpr::NodeExprIdent(expr_ident) => {

            },
            _ => {

            }
        }

        return "".to_string();
    }

    pub fn generate_statement(&mut self, stmt: &NodeStatement) -> String {
        println!("I am generating statement");
        match stmt {
            NodeStatement::NodeStatementExitEnum(stmt_exit) => {
                self.generate_expression(&stmt_exit.expr);
                fmt::Write::write_str(&mut self.stream, "    mov rax, 60\n").unwrap();
                self.pop("rdi");
            },
            NodeStatement::NodeStatementLetEnum(stmt_let) => {

            },
            _ => {

            }
        }
        return " ".to_string();
    }

    pub fn generate_program(&mut self) -> String {
        self.stream = String::from("global _start\n_start:\n");

        // Iterate over a reference to the vector's content
        for stmt in &self.m_program.statements {
            generate_statement(stmt);
            println!("stmt is {:?}", stmt);
        }

        fmt::Write::write_str(&mut self.stream, "    mov rax, 60\n").unwrap();
        fmt::Write::write_str(&mut self.stream, "    mov rdi, 42\n").unwrap();
        fmt::Write::write_str(&mut self.stream, "    syscall\n").unwrap();
        self.stream.clone()
    }
}

pub fn generate_statement(stmt: &NodeStatement) -> String {
    match stmt {
        NodeStatement::NodeStatementExitEnum(stmt_exit) => {
            println!("Exit enum");
        },
        NodeStatement::NodeStatementLetEnum(stmt_let) => {
            println!("Statement enum");
        },
        _ => {
            println!("What am i ");
        }
    }
    return " ".to_string();
}
