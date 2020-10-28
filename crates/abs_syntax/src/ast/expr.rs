use std::fmt;

use crate::{add_fmt, fmt::ABSFormatter};

use super::{CaseBranch, DisplayABS, Ident, Literal, Type};
#[derive(Clone)]
pub enum Expr {
    Pure(PureExpr),
    Eff(EffExpr),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for Expr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        match self {
            Expr::Pure(e) => e.to_abs(f),
            Expr::Eff(e) => e.to_abs(f),
        }
    }
}

impl<E: Into<PureExpr>> From<E> for Expr {
    fn from(e: E) -> Self {
        let e: PureExpr = e.into();
        Expr::Pure(e)
    }
}

impl From<EffExpr> for Expr {
    fn from(e: EffExpr) -> Self {
        Expr::Eff(e)
    }
}

#[derive(Clone)]
pub enum PureExpr {
    Ident(IdentExpr),
    ThisIdent(IdentExpr),
    This,
    Null,
    Literal(Literal),
    TemplateString,
    Let(LetExpr),
    DataConstr(DataConstrExpr),
    FnApp(FnAppExpr),
    ParFnApp(ParFnAppExpr),
    When(WhenExpr),
    Case(CaseExpr),
    Operator(OperatorExpr),
    TypeCheck(TypeCheckExpr),
    TypeCast(TypeCastExpr),
}

impl DisplayABS for PureExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        match self {
            PureExpr::Ident(e) => e.to_abs(f),
            PureExpr::ThisIdent(e) => e.to_abs(f),
            PureExpr::This => f.add("this"),
            PureExpr::Null => f.add("null"),
            PureExpr::Literal(e) => e.to_abs(f),
            PureExpr::TemplateString => todo!(),
            PureExpr::Let(e) => e.to_abs(f),
            PureExpr::DataConstr(e) => e.to_abs(f),
            PureExpr::FnApp(e) => e.to_abs(f),
            PureExpr::ParFnApp(e) => e.to_abs(f),
            PureExpr::When(e) => e.to_abs(f),
            PureExpr::Case(e) => e.to_abs(f),
            PureExpr::Operator(e) => e.to_abs(f),
            PureExpr::TypeCheck(e) => e.to_abs(f),
            PureExpr::TypeCast(e) => e.to_abs(f),
        }
    }
}

impl fmt::Display for PureExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PureExpr::Ident(i) => fmt::Display::fmt(i, f),
            PureExpr::ThisIdent(i) => write!(f, "this.{}", i),
            PureExpr::This => write!(f, "this"),
            PureExpr::Literal(i) => fmt::Display::fmt(i, f),
            PureExpr::TemplateString => todo!(),
            PureExpr::Let(i) => fmt::Display::fmt(i, f),
            PureExpr::DataConstr(i) => fmt::Display::fmt(i, f),
            PureExpr::FnApp(i) => fmt::Display::fmt(i, f),
            PureExpr::ParFnApp(i) => fmt::Display::fmt(i, f),
            PureExpr::When(i) => fmt::Display::fmt(i, f),
            PureExpr::Case(i) => fmt::Display::fmt(i, f),
            PureExpr::Operator(i) => fmt::Display::fmt(i, f),
            PureExpr::TypeCheck(i) => fmt::Display::fmt(i, f),
            PureExpr::TypeCast(i) => fmt::Display::fmt(i, f),
            PureExpr::Null => write!(f, "null"),
        }
    }
}

impl From<Literal> for PureExpr {
    fn from(e: Literal) -> Self {
        PureExpr::Literal(e)
    }
}

impl From<IdentExpr> for PureExpr {
    fn from(e: IdentExpr) -> Self {
        PureExpr::Ident(e)
    }
}

#[derive(Clone)]
pub struct IdentExpr {
    pub ident: Ident,
}

impl fmt::Display for IdentExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for IdentExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        self.ident.to_abs(f)
    }
}

#[derive(Clone)]
pub struct LetExpr {
    pub ty: Type,
    pub ident: Ident,
    pub value: Box<PureExpr>,
    pub inner: Box<PureExpr>,
}

impl fmt::Display for LetExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for LetExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("let ");
        self.ty.to_abs(f);
        f.add(" ");
        self.ident.to_abs(f);
        f.add(" = ");
        self.value.to_abs(f);
        f.add_indent();
        f.new_line();
        f.add("in ");
        self.inner.to_abs(f);
        f.sub_indent();
    }
}

#[derive(Clone)]
pub struct DataConstrExpr {
    pub ident: Ident,
    pub args: Vec<PureExpr>,
}

impl fmt::Display for DataConstrExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for DataConstrExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        self.ident.to_abs(f);
        f.parenthesized(|f| f.list(self.args.iter(), ", "))
    }
}

#[derive(Clone)]
pub struct FnAppExpr {
    pub ident: Ident,
    pub args: Vec<PureExpr>,
}

impl fmt::Display for FnAppExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for FnAppExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        self.ident.to_abs(f);
        f.parenthesized(|f| f.list(self.args.iter(), ", "))
    }
}

#[derive(Clone)]
pub struct ParFnAppExpr {}

impl fmt::Display for ParFnAppExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for ParFnAppExpr {
    fn to_abs(&self, _f: &mut crate::fmt::ABSFormatter) {
        todo!()
    }
}

#[derive(Clone)]
pub struct WhenExpr {
    pub condition: Box<PureExpr>,
    pub then: Box<PureExpr>,
    pub r#else: Box<PureExpr>,
}

impl fmt::Display for WhenExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for WhenExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("when ");
        self.condition.to_abs(f);
        f.add(" then ");
        self.then.to_abs(f);
        f.add(" else ");
        self.r#else.to_abs(f)
    }
}

#[derive(Clone)]
pub struct CaseExpr {
    pub expr: Box<PureExpr>,
    pub branches: Vec<CaseBranch<PureExpr>>,
}

impl fmt::Display for CaseExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for CaseExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("case ");
        self.expr.to_abs(f);
        f.braced(|f| {
            f.list_fn(
                self.branches.iter(),
                |i, f| {
                    if i > 0 {
                        f.new_line();
                        f.add("|")
                    }
                },
                |_, _| {},
            )
        })
    }
}

#[derive(Clone)]
pub struct TypeCheckExpr {
    pub expr: Box<PureExpr>,
    pub ty: Ident,
}

impl fmt::Display for TypeCheckExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for TypeCheckExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        self.expr.to_abs(f);
        f.add(" implements ");
        self.ty.to_abs(f)
    }
}

#[derive(Clone)]
pub struct TypeCastExpr {
    pub expr: Box<PureExpr>,
    pub ty: Ident,
}

impl fmt::Display for TypeCastExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for TypeCastExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        self.expr.to_abs(f);
        f.add(" as ");
        self.ty.to_abs(f)
    }
}

#[derive(Clone)]
pub enum OperatorExpr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
}

impl fmt::Display for OperatorExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for OperatorExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        match self {
            OperatorExpr::Unary(e) => e.to_abs(f),
            OperatorExpr::Binary(e) => e.to_abs(f),
        }
    }
}

#[derive(Clone)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<PureExpr>,
}

impl fmt::Display for UnaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for UnaryExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        add_fmt!(f, "{} ", self.op);
        self.expr.to_abs(f);
    }
}

#[derive(Clone)]
pub enum UnaryOp {
    Not,
    Minus,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Not => write!(f, "!"),
            UnaryOp::Minus => write!(f, "-"),
        }
    }
}

#[derive(Clone)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub left: Box<PureExpr>,
    pub right: Box<PureExpr>,
}

impl fmt::Display for BinaryExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for BinaryExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        self.left.to_abs(f);
        add_fmt!(f, " {} ", self.op);
        self.right.to_abs(f);
    }
}

#[derive(Clone, Copy)]
pub enum BinaryOp {
    Or,
    And,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Plus,
    Minus,
    Mult,
    Div,
    Mod,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOp::Or => "||",
            BinaryOp::And => "&&",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",
            BinaryOp::Plus => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Mult => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
        };
        fmt::Display::fmt(s, f)
    }
}

#[derive(Clone)]
pub enum EffExpr {
    New(NewExpr),
    SyncCall(SyncCallExpr),
    AsyncCall(AsyncCallExpr),
    Get(GetExpr),
    Await(AwaitExpr),
}

impl fmt::Display for EffExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for EffExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        match self {
            EffExpr::New(e) => e.to_abs(f),
            EffExpr::SyncCall(e) => e.to_abs(f),
            EffExpr::AsyncCall(e) => e.to_abs(f),
            EffExpr::Get(e) => e.to_abs(f),
            EffExpr::Await(e) => e.to_abs(f),
        }
    }
}

impl From<NewExpr> for EffExpr {
    fn from(e: NewExpr) -> Self {
        EffExpr::New(e)
    }
}

#[derive(Clone)]
pub struct NewExpr {
    pub local: bool,
    pub ty: Ident,
    pub args: Vec<PureExpr>,
}

impl fmt::Display for NewExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for NewExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("new ");
        if self.local {
            f.add("local ");
        }
        self.ty.to_abs(f);
        f.parenthesized(|f| f.list(self.args.iter(), ", "))
    }
}

#[derive(Clone)]
pub struct SyncCallExpr {
    pub callee: PureExpr,
    pub method: Ident,
    pub args: Vec<PureExpr>,
}

impl fmt::Display for SyncCallExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for SyncCallExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        self.callee.to_abs(f);
        f.add(".");
        self.method.to_abs(f);
        f.parenthesized(|f| f.list(self.args.iter(), ", "))
    }
}

#[derive(Clone)]
pub struct AsyncCallExpr {
    pub callee: PureExpr,
    pub method: Ident,
    pub args: Vec<PureExpr>,
}

impl fmt::Display for AsyncCallExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for AsyncCallExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        self.callee.to_abs(f);
        f.add("!");
        self.method.to_abs(f);
        f.parenthesized(|f| f.list(self.args.iter(), ", "))
    }
}

#[derive(Clone)]
pub struct GetExpr {
    pub expr: PureExpr,
}

impl fmt::Display for GetExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for GetExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        self.expr.to_abs(f);
        f.add(".get")
    }
}

#[derive(Clone)]
pub struct AwaitExpr {
    pub call: AsyncCallExpr,
}

impl fmt::Display for AwaitExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for AwaitExpr {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("await ");
        self.call.to_abs(f)
    }
}
