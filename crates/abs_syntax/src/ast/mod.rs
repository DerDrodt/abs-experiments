use std::fmt;

mod expr;
mod guard;
mod ident;
mod lit;
mod pattern;
mod stmt;
mod ty;

pub use expr::*;
pub use guard::*;
pub use ident::*;
pub use lit::*;
pub use pattern::*;
pub use stmt::*;
pub use ty::*;

use crate::fmt::ABSFormatter;

pub trait DisplayABS {
    fn to_abs(&self, f: &mut ABSFormatter);
}

impl<T> DisplayABS for &T
where
    T: DisplayABS + Clone,
{
    fn to_abs(&self, f: &mut ABSFormatter) {
        self.clone().to_abs(f)
    }
}

#[macro_export]
macro_rules! add_fmt {
    ($f:expr, $s:expr, $($arg:expr),*) => {
        $f.add(&format!($s, $($arg),*))
    };
}

#[derive(Clone)]
pub struct Module {
    pub name: Ident,
    pub children: Vec<ModuleItem>,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for Module {
    fn to_abs(&self, f: &mut ABSFormatter) {
        add_fmt!(f, "module {};", self.name);
        f.new_line();
        f.new_line();

        for c in &self.children {
            c.to_abs(f);
            f.new_line();
            f.new_line()
        }
    }
}

#[derive(Clone)]
pub enum ModuleItem {
    InterfaceDecl(InterfaceDecl),
    ClassDecl(ClassDecl),
    MainBlock(Block),
}

impl fmt::Display for ModuleItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for ModuleItem {
    fn to_abs(&self, f: &mut ABSFormatter) {
        match self {
            ModuleItem::InterfaceDecl(i) => i.to_abs(f),
            ModuleItem::ClassDecl(c) => c.to_abs(f),
            ModuleItem::MainBlock(b) => b.to_abs(f),
        }
    }
}

impl From<InterfaceDecl> for ModuleItem {
    fn from(i: InterfaceDecl) -> Self {
        ModuleItem::InterfaceDecl(i)
    }
}

impl From<ClassDecl> for ModuleItem {
    fn from(i: ClassDecl) -> Self {
        ModuleItem::ClassDecl(i)
    }
}

impl From<Block> for ModuleItem {
    fn from(i: Block) -> Self {
        ModuleItem::MainBlock(i)
    }
}

#[derive(Clone)]
pub struct InterfaceDecl {
    pub ident: Ident,
    pub extends: Vec<Ident>,
    pub sigs: Vec<MethodSig>,
}

impl fmt::Display for InterfaceDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for InterfaceDecl {
    fn to_abs(&self, f: &mut ABSFormatter) {
        f.add("interface ");
        self.ident.to_abs(f);
        f.add(" ");
        if !self.extends.is_empty() {
            f.add("extends ");
            f.list(self.extends.iter(), ", ");
            f.add(" ");
        }
        f.braced(|f| {
            f.list_fn(
                self.sigs.iter(),
                |i, f| {
                    if i > 0 {
                        f.new_line();
                    }
                },
                |_, f| f.add(";"),
            )
        })
    }
}

#[derive(Clone)]
pub struct ClassDecl {
    pub ident: Ident,
    pub params: Vec<Param>,
    pub implements: Vec<Ident>,
    pub fields: Vec<FieldDecl>,
    pub init: Option<Block>,
    pub recover: Vec<CaseBranch<Stmt>>,
    pub methods: Vec<MethodDecl>,
}

impl fmt::Display for ClassDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for ClassDecl {
    fn to_abs(&self, f: &mut ABSFormatter) {
        f.add("class ");
        self.ident.to_abs(f);
        if !self.params.is_empty() {
            f.parenthesized(|f| f.list(self.params.iter(), ", "));
        }
        if !self.implements.is_empty() {
            f.add(" implements ");
            f.list(self.implements.iter(), ", ");
        }
        f.add(" ");
        f.braced(|f| {
            f.list_fn(self.fields.iter(), |_, _| {}, |_, f| f.new_line());
            if let Some(init) = &self.init {
                init.to_abs(f);
                f.new_line();
            }

            if !self.recover.is_empty() {
                f.add("recover ");
                f.braced(|f| {
                    f.list_fn(
                        self.recover.iter(),
                        |i, f| {
                            if i > 0 {
                                f.new_line()
                            }
                        },
                        |_, f| f.add(";"),
                    )
                })
            }

            f.list_fn(
                self.methods.iter(),
                |i, f| {
                    if i > 0 {
                        f.new_line();
                        f.new_line();
                    }
                },
                |_, _| {},
            )
        });
    }
}

#[derive(Clone)]
pub struct MethodSig {
    pub ret: Type,
    pub ident: Ident,
    pub params: Vec<Param>,
}

impl fmt::Display for MethodSig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for MethodSig {
    fn to_abs(&self, f: &mut ABSFormatter) {
        self.ret.to_abs(f);
        f.add(" ");
        self.ident.to_abs(f);
        f.parenthesized(|f| f.list(self.params.iter(), ", "))
    }
}

#[derive(Clone)]
pub struct Param {
    pub ty: Type,
    pub ident: Ident,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for Param {
    fn to_abs(&self, f: &mut ABSFormatter) {
        self.ty.to_abs(f);
        f.add(" ");
        self.ident.to_abs(f);
    }
}

#[derive(Clone)]
pub struct FieldDecl {
    pub ty: Type,
    pub ident: Ident,
    pub init: Option<PureExpr>,
}

impl fmt::Display for FieldDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for FieldDecl {
    fn to_abs(&self, f: &mut ABSFormatter) {
        self.ty.to_abs(f);
        f.add(" ");
        self.ident.to_abs(f);
        if let Some(e) = &self.init {
            f.add(" = ");
            e.to_abs(f);
        }
        f.add(";")
    }
}

#[derive(Clone)]
pub struct MethodDecl {
    pub sig: MethodSig,
    pub body: Block,
}

impl fmt::Display for MethodDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut af = ABSFormatter::new();
        self.to_abs(&mut af);
        fmt::Display::fmt(&af.abs_code(), f)
    }
}

impl DisplayABS for MethodDecl {
    fn to_abs(&self, f: &mut ABSFormatter) {
        self.sig.to_abs(f);
        f.add(" ");
        self.body.to_abs(f);
    }
}
