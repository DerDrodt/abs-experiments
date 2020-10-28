use std::fmt;

use super::*;

pub struct Type {
    pub ident: Ident,
    pub args: Vec<Type>,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut args = String::new();
        for (i, a) in self.args.iter().enumerate() {
            if i == 0 {
                args.push_str("<");
            }
            if i > 0 {
                args.push_str(", ");
            }
            args.push_str(&a.to_string());
        }
        if !self.args.is_empty() {
            args.push('>');
        }
        write!(f, "{}{}", self.ident, args)
    }
}
