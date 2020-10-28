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

pub trait DisplayABS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, indent: u32) -> fmt::Result;
}

pub struct Module {
    pub name: Ident,
    pub children: Vec<ModuleItem>,
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut children = String::new();

        for c in &self.children {
            children.push_str(&format!("\n{}\n", c))
        }

        write!(f, "module {};\n{}", self.name, children)
    }
}

pub enum ModuleItem {
    InterfaceDecl(InterfaceDecl),
    ClassDecl(ClassDecl),
    MainBlock(Block),
}

impl fmt::Display for ModuleItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleItem::InterfaceDecl(i) => fmt::Display::fmt(i, f),
            ModuleItem::ClassDecl(c) => fmt::Display::fmt(c, f),
            ModuleItem::MainBlock(b) => fmt::Display::fmt(b, f),
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

pub struct InterfaceDecl {
    pub ident: Ident,
    pub extends: Vec<Ident>,
    pub sigs: Vec<MethodSig>,
}

impl fmt::Display for InterfaceDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut extends = String::new();

        for (i, e) in self.extends.iter().enumerate() {
            if i == 0 {
                extends.push_str(" extends ");
            }
            if i > 0 {
                extends.push_str(", ");
            }
            extends.push_str(&e.to_string());
        }

        let mut sigs = String::new();

        for s in &self.sigs {
            sigs.push_str("\n\t");
            sigs.push_str(&s.to_string());
            sigs.push_str(";")
        }

        sigs.push('\n');

        write!(f, "interface {}{} {{{}}}", self.ident, extends, sigs)
    }
}

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
        let mut params = String::new();
        if !self.params.is_empty() {
            params.push_str("(");
            for (i, p) in self.params.iter().enumerate() {
                if i > 0 {
                    params.push_str(", ");
                }
                params.push_str(&p.to_string())
            }
            params.push_str(")")
        }

        let mut implements = String::new();
        if !self.implements.is_empty() {
            implements.push_str(" implements ");
            for (i, p) in self.implements.iter().enumerate() {
                if i > 0 {
                    implements.push_str(", ");
                }
                implements.push_str(&p.to_string())
            }
        }

        let fields = if self.fields.is_empty() {
            String::new()
        } else {
            let mut s = String::new();
            for f in &self.fields {
                s.push_str(&format!("\t{}\n", f));
            }
            s
        };

        let init = match &self.init {
            Some(i) => format!("\n{}\n", i),
            _ => String::new(),
        };

        let recover = if self.recover.is_empty() {
            String::new()
        } else {
            let mut s = "recover {{\n".to_string();
            for b in &self.recover {
                s.push_str(&format!("\t{}\n", b));
            }
            s.push_str("}}\n");
            s
        };

        let methods = if self.methods.is_empty() {
            String::new()
        } else {
            let mut s = String::new();
            for (i, m) in self.methods.iter().enumerate() {
                if i > 0 {
                    s.push_str("\n\n");
                }
                s.push_str(&m.to_string());
            }
            s
        };

        write!(
            f,
            "class {}{}{} {{\n{}{}{}{}}}",
            self.ident, params, implements, fields, init, recover, methods
        )
    }
}

pub struct MethodSig {
    pub ret: Type,
    pub ident: Ident,
    pub params: Vec<Param>,
}

impl fmt::Display for MethodSig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut params = String::new();
        for (i, p) in self.params.iter().enumerate() {
            if i > 0 {
                params.push_str(", ");
            }
            params.push_str(&p.to_string());
        }

        write!(f, "\t{} {}({})", self.ret, self.ident, params)
    }
}

pub struct Param {
    pub ty: Type,
    pub ident: Ident,
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.ty, self.ident)
    }
}

pub struct FieldDecl {
    pub ty: Type,
    pub ident: Ident,
    pub init: Option<PureExpr>,
}

impl fmt::Display for FieldDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let init = match &self.init {
            Some(e) => format!(" = {}", e),
            _ => "".to_string(),
        };
        write!(f, "{} {}{};", self.ty, self.ident, init)
    }
}

pub struct MethodDecl {
    pub sig: MethodSig,
    pub body: Block,
}

impl fmt::Display for MethodDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.sig, self.body)
    }
}
