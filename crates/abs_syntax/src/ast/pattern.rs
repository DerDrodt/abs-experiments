use std::fmt;

use super::DisplayABS;

#[derive(Clone)]
pub struct CaseBranch<K> {
    pub pattern: Pattern,
    pub right: K,
}

impl<K> fmt::Display for CaseBranch<K>
where
    K: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.pattern, self.right)
    }
}

impl<K> DisplayABS for CaseBranch<K>
where
    K: DisplayABS,
{
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        self.pattern.to_abs(f);
        f.add(" => ");
        self.right.to_abs(f);
    }
}

#[derive(Clone)]
pub struct Pattern;

impl fmt::Display for Pattern {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl DisplayABS for Pattern {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        todo!()
    }
}
