use std::fmt;

use crate::fmt::ABSFormatter;

use super::{Annotations, CaseBranch, DisplayABS, Expr, Guard, Ident, PureExpr, Type};

#[derive(Clone)]
pub enum Stmt {
    Skip,
    VarDecl(VarDeclStmt),
    Assign(AssignStmt),
    Expr(ExprStmt),
    Assert(AssertStmt),
    Await(AwaitStmt),
    Suspend,
    Throw(ThrowStmt),
    Return(ReturnStmt),
    Block(Block),
    If(IfStmt),
    Switch(SwitchStmt),
    While(WhileStmt),
    Foreach(ForeachStmt),
    TryCatchFinally(TryCatchFinallyStmt),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for Stmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.stmt(|f| match self {
            Stmt::Skip => f.add("skip;"),
            Stmt::VarDecl(s) => s.to_abs(f),
            Stmt::Assign(s) => s.to_abs(f),
            Stmt::Expr(s) => s.to_abs(f),
            Stmt::Assert(s) => s.to_abs(f),
            Stmt::Await(s) => s.to_abs(f),
            Stmt::Suspend => f.add("suspend;"),
            Stmt::Throw(s) => s.to_abs(f),
            Stmt::Return(s) => s.to_abs(f),
            Stmt::Block(s) => s.to_abs(f),
            Stmt::If(s) => s.to_abs(f),
            Stmt::Switch(s) => s.to_abs(f),
            Stmt::While(s) => s.to_abs(f),
            Stmt::Foreach(s) => s.to_abs(f),
            Stmt::TryCatchFinally(s) => s.to_abs(f),
        })
    }
}

impl From<VarDeclStmt> for Stmt {
    fn from(s: VarDeclStmt) -> Self {
        Stmt::VarDecl(s)
    }
}

impl From<AssignStmt> for Stmt {
    fn from(s: AssignStmt) -> Self {
        Stmt::Assign(s)
    }
}

impl From<ExprStmt> for Stmt {
    fn from(s: ExprStmt) -> Self {
        Stmt::Expr(s)
    }
}

impl From<IfStmt> for Stmt {
    fn from(s: IfStmt) -> Self {
        Stmt::If(s)
    }
}

impl From<WhileStmt> for Stmt {
    fn from(s: WhileStmt) -> Self {
        Stmt::While(s)
    }
}

impl From<Block> for Stmt {
    fn from(s: Block) -> Self {
        Stmt::Block(s)
    }
}

impl From<ReturnStmt> for Stmt {
    fn from(s: ReturnStmt) -> Self {
        Stmt::Return(s)
    }
}

#[derive(Clone)]
pub struct VarDeclStmt {
    pub annotations: Annotations,
    pub ty: Type,
    pub ident: Ident,
    pub init: Option<Expr>,
}

impl fmt::Display for VarDeclStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for VarDeclStmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        self.annotations.to_abs(f);
        self.ty.to_abs(f);
        f.add(" ");
        self.ident.to_abs(f);
        if let Some(init) = &self.init {
            f.add(" = ");
            init.to_abs(f);
        }
        f.add(";")
    }
}

#[derive(Clone)]
pub struct AssignStmt {
    pub this: bool,
    pub ident: Ident,
    pub expr: Expr,
}

impl fmt::Display for AssignStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for AssignStmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        if self.this {
            f.add("this.");
        }
        self.ident.to_abs(f);
        f.add(" = ");
        self.expr.to_abs(f);
        f.add(";")
    }
}

#[derive(Clone)]
pub struct ExprStmt {
    pub expr: Expr,
}

impl fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for ExprStmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        self.expr.to_abs(f);
        f.add(";")
    }
}

#[derive(Clone)]
pub struct AssertStmt {
    pub condition: PureExpr,
}

impl fmt::Display for AssertStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for AssertStmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("assert ");
        self.condition.to_abs(f);
        f.add(";")
    }
}

#[derive(Clone)]
pub struct AwaitStmt {
    pub guard: Guard,
}

impl fmt::Display for AwaitStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for AwaitStmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("await ");
        self.guard.to_abs(f);
        f.add(";")
    }
}

#[derive(Clone)]
pub struct ReturnStmt {
    pub expr: Expr,
}

impl fmt::Display for ReturnStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for ReturnStmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("return ");
        self.expr.to_abs(f);
        f.add(";")
    }
}

#[derive(Clone)]
pub struct ThrowStmt {
    pub expr: PureExpr,
}

impl fmt::Display for ThrowStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for ThrowStmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("throw ");
        self.expr.to_abs(f);
        f.add(";")
    }
}

#[derive(Clone)]
pub struct IfStmt {
    pub condition: PureExpr,
    pub then: Box<Stmt>,
    pub r#else: Option<Box<Stmt>>,
}

impl fmt::Display for IfStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for IfStmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("if ");
        f.parenthesized(|f| self.condition.to_abs(f));
        f.add(" ");
        self.then.to_abs(f);
        if let Some(e) = &self.r#else {
            f.add(" else ");
            e.to_abs(f);
        }
    }
}

#[derive(Clone)]
pub struct SwitchStmt {
    pub expr: PureExpr,
    pub branches: Vec<CaseBranch<Stmt>>,
}

impl fmt::Display for SwitchStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for SwitchStmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("switch ");
        self.expr.to_abs(f);
        f.add(" ");
        f.braced(|f| {
            f.list_fn(
                self.branches.iter(),
                |i, f| {
                    if i > 0 {
                        f.new_line()
                    }
                },
                |_, _| {},
            )
        })
    }
}

#[derive(Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for Block {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.block();
        f.braced(|f| {
            f.list_fn(
                self.stmts.iter(),
                |i, f| {
                    if i > 0 {
                        f.new_line()
                    }
                },
                |_, _| {},
            )
        })
    }
}

#[derive(Clone)]
pub struct WhileStmt {
    pub condition: PureExpr,
    pub body: Box<Stmt>,
}

impl fmt::Display for WhileStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for WhileStmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("while ");
        f.parenthesized(|f| self.condition.to_abs(f));
        f.add(" ");
        self.body.to_abs(f)
    }
}

#[derive(Clone)]
pub struct ForeachStmt {
    pub loop_var: Ident,
    pub iter: PureExpr,
    pub body: Box<Stmt>,
}

impl fmt::Display for ForeachStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for ForeachStmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("foreach ");
        f.parenthesized(|f| {
            self.loop_var.to_abs(f);
            f.add(" in ");
            self.iter.to_abs(f)
        });
        f.add(" ");
        self.body.to_abs(f)
    }
}

#[derive(Clone)]
pub struct TryCatchFinallyStmt {
    pub r#try: Box<Stmt>,
    pub catch_branches: Vec<CaseBranch<Stmt>>,
    pub finally: Option<Box<Stmt>>,
}

impl fmt::Display for TryCatchFinallyStmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for TryCatchFinallyStmt {
    fn to_abs(&self, f: &mut crate::fmt::ABSFormatter) {
        f.add("try ");
        self.r#try.to_abs(f);
        f.add(" catch ");
        f.braced(|f| {
            f.list_fn(
                self.catch_branches.iter(),
                |i, f| {
                    if i > 0 {
                        f.new_line()
                    }
                },
                |_, _| {},
            )
        });
        if let Some(finally) = &self.finally {
            f.add(" finally ");
            finally.to_abs(f);
        }
    }
}
