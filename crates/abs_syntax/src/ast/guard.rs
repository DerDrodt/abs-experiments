use std::fmt;

use crate::fmt::ABSFormatter;

use super::{DisplayABS, Ident, PureExpr};

#[derive(Clone)]
pub enum Guard {
    Claim { this: bool, ident: Ident },
    Expr(PureExpr),
    And(Box<Guard>, Box<Guard>),
    Duration(PureExpr, PureExpr),
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for Guard {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        match self {
            Guard::Claim { this, ident } => {
                if *this {
                    f.add("this.")
                }
                ident.to_abs(f);
                f.add("?")
            }
            Guard::Expr(e) => e.to_abs(f),
            Guard::And(l, r) => {
                l.to_abs(f);
                f.add(" & ");
                r.to_abs(f);
            }
            Guard::Duration(min, max) => {
                f.add("duration(");
                min.to_abs(f);
                f.add(", ");
                max.to_abs(f);
                f.add(")")
            }
        }
    }
}
