use std::fmt;

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
        match self {
            Guard::Claim { this, ident } => {
                if *this {
                    write!(f, "this.{}?", ident)
                } else {
                    write!(f, "{}?", ident)
                }
            }
            Guard::Expr(e) => fmt::Display::fmt(e, f),
            Guard::And(l, r) => write!(f, "{} & {}", l, r),
            Guard::Duration(min, max) => write!(f, "duration({},{})", min, max),
        }
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
