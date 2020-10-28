use std::fmt;

use super::DisplayABS;

#[derive(Clone)]
pub struct Ident {
    pub str: String,
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.str, f)
    }
}

impl DisplayABS for Ident {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add(&self.str)
    }
}
