use abs_syntax::ast;

use super::ident;

pub struct TypeBuilder {
    ident: ast::Ident,
    args: Vec<ast::Type>,
}

impl TypeBuilder {
    pub fn new(name: String) -> Self {
        TypeBuilder {
            ident: ident(name),
            args: vec![],
        }
    }

    pub fn add_args(&mut self, ty: ast::Type) {
        self.args.push(ty);
    }

    pub fn with_args(mut self, ty: ast::Type) -> Self {
        self.add_args(ty);
        self
    }

    pub fn complete(self) -> ast::Type {
        ast::Type {
            ident: self.ident,
            args: self.args,
        }
    }
}

pub fn start_type<S: Into<String>>(name: S) -> TypeBuilder {
    TypeBuilder::new(name.into())
}

pub fn simple_ty<S: Into<String>>(name: S) -> ast::Type {
    start_type(name).complete()
}

pub fn create_unit() -> ast::Type {
    simple_ty("Unit")
}

pub fn create_int() -> ast::Type {
    simple_ty("Int")
}

pub fn create_bool() -> ast::Type {
    simple_ty("Bool")
}

pub fn create_fut(ty: ast::Type) -> ast::Type {
    start_type("Fut").with_args(ty).complete()
}

pub fn create_i() -> ast::Type {
    simple_ty("I")
}

pub fn create_j() -> ast::Type {
    simple_ty("J")
}
