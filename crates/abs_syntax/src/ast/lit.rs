use std::fmt;

use super::DisplayABS;

#[derive(Clone)]
pub struct Literal {
    pub s: String,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.s, f)
    }
}

impl DisplayABS for Literal {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add(&self.s)
    }
}
