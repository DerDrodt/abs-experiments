use super::*;

#[derive(Clone)]
pub struct Annotations(Vec<Annotation>);

impl Annotations {
    pub fn push(&mut self, a: Annotation) {
        self.0.push(a)
    }
}

impl Default for Annotations {
    fn default() -> Self {
        Annotations(Vec::new())
    }
}

impl DisplayABS for Annotations {
    fn to_abs(&self, f: &mut ABSFormatter) {
        f.list(self.0.iter(), " ")
    }
}

#[derive(Clone)]
pub enum Annotation {
    Typed(TypedAnnotation),
    Untyped(UntypedAnnotation),
}

impl DisplayABS for Annotation {
    fn to_abs(&self, f: &mut ABSFormatter) {
        match self {
            Annotation::Typed(a) => a.to_abs(f),
            Annotation::Untyped(a) => a.to_abs(f),
        }
    }
}

#[derive(Clone)]
pub struct TypedAnnotation {
    pub ty: Type,
    pub expr: PureExpr,
}

impl DisplayABS for TypedAnnotation {
    fn to_abs(&self, f: &mut ABSFormatter) {
        f.bracketed(|f| {
            self.ty.to_abs(f);
            f.add(": ");
            self.expr.to_abs(f)
        })
    }
}

#[derive(Clone)]
pub struct UntypedAnnotation(pub PureExpr);

impl DisplayABS for UntypedAnnotation {
    fn to_abs(&self, f: &mut ABSFormatter) {
        f.bracketed(|f| self.0.to_abs(f));
    }
}
