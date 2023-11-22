use crate::parser::NodeExit;
use std::fmt;

pub struct Generator {
    m_root: NodeExit,
}

impl Generator {
    pub fn new(m_root: NodeExit) -> Self {
        Self { m_root }
    }

    pub fn generate(&mut self) -> String {
        let mut stream = String::from("global _start\n_start:\n");
        fmt::Write::write_str(&mut stream, "    mov rax, 60\n").unwrap();
        fmt::Write::write_str(
            &mut stream,
            &format!(
                "    mov rdi, {:?}\n",
                self.m_root.expr.int_lit.value.as_ref().unwrap().parse::<i32>().unwrap()
            ),
        )
        .unwrap();
        fmt::Write::write_str(&mut stream, "    syscall\n").unwrap();

        return stream;
    }
}
