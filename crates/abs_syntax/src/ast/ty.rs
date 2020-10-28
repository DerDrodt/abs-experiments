use std::fmt;

use super::*;

#[derive(Clone)]
pub struct Type {
    pub ident: Ident,
    pub args: Vec<Type>,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for Type {
    fn to_abs(&self, f: &mut ABSFormatter) {
        self.ident.to_abs(f);
        if !self.args.is_empty() {
            f.angle_bracketed(|f| f.list(self.args.iter(), ", "));
        }
    }
}
