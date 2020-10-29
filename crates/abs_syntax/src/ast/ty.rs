use rand::seq::SliceRandom;
use std::fmt;

use super::*;

#[derive(Clone)]
pub struct Type {
    pub ident: Ident,
    pub args: Vec<Type>,
}

impl Type {
    pub fn is_bool(&self) -> bool {
        self.ident.str == "Bool"
    }

    pub fn is_unit(&self) -> bool {
        self.ident.str == "Unit"
    }

    pub fn is_int(&self) -> bool {
        self.ident.str == "Int"
    }

    pub fn is_fut(&self) -> bool {
        self.ident.str == "Fut"
    }

    pub fn has_lit(&self) -> bool {
        self.is_int() || self.is_bool()
    }

    pub fn get_lit(&self) -> Option<Literal> {
        if self.is_int() {
            let n = (rand::random::<f64>() * 1000.0) as i32 - 500;
            Some(Literal { s: n.to_string() })
        } else if self.is_bool() {
            Some(Literal {
                s: ["True", "False"]
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .to_string(),
            })
        } else {
            None
        }
    }
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
