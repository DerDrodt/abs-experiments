use crate::ast::DisplayABS;

pub struct ABSFormatter {
    indent: u32,
    out: String,
    break_on_stmt: bool,
}

impl ABSFormatter {
    pub fn new() -> Self {
        Self {
            indent: 0,
            out: String::new(),
            break_on_stmt: true,
        }
    }

    pub fn add_indent(&mut self) {
        self.indent += 1;
    }

    pub fn sub_indent(&mut self) {
        if self.indent == 0 {
            panic!("Negative indentation!")
        }
        self.indent -= 1;
    }

    pub fn add(&mut self, s: &str) {
        self.out.push_str(s)
    }

    pub fn indent_str(&self) -> String {
        "\t".repeat(self.indent as usize)
    }

    pub fn new_line(&mut self) {
        self.add(&format!("\n{}", self.indent_str()))
    }

    pub fn abs_code(self) -> String {
        self.out
    }

    pub fn list<I, E>(&mut self, lst: I, sep: &str)
    where
        I: Iterator<Item = E>,
        E: DisplayABS,
    {
        for (i, e) in lst.enumerate() {
            if i > 0 {
                self.add(sep);
            }
            e.to_abs(self);
        }
    }

    pub fn list_fn<I, E, F1, F2>(&mut self, lst: I, mut before: F1, mut after: F2)
    where
        I: Iterator<Item = E>,
        E: DisplayABS,
        F1: FnMut(usize, &mut ABSFormatter),
        F2: FnMut(usize, &mut ABSFormatter),
    {
        for (i, e) in lst.enumerate() {
            before(i, self);
            e.to_abs(self);
            after(i, self);
        }
    }

    pub fn parenthesized<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ABSFormatter),
    {
        self.add("(");
        f(self);
        self.add(")");
    }

    pub fn braced<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ABSFormatter),
    {
        self.add("{");
        self.add_indent();
        self.new_line();
        f(self);
        self.sub_indent();
        self.new_line();
        self.add("}");
    }

    pub fn angle_bracketed<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ABSFormatter),
    {
        self.add("<");
        f(self);
        self.add(">");
    }

    pub fn bracketed<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ABSFormatter),
    {
        self.add("[");
        f(self);
        self.add("]");
    }

    pub fn stmt<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ABSFormatter),
    {
        f(self)
    }

    pub fn start_stmt(&mut self) {
        if self.break_on_stmt {
            self.new_line();
        }
    }

    pub fn block(&mut self) {}
}
