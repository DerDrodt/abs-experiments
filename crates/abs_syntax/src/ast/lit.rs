use std::fmt;

use crate::fmt::ABSFormatter;

use super::DisplayABS;

#[derive(Clone)]
pub struct Literal {
    pub s: String,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for Literal {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add(&self.s)
    }
}
