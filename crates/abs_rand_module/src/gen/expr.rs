use abs_syntax::ast;

use super::ident;

pub fn create_var_use<S: Into<String>>(v: S) -> ast::IdentExpr {
    ast::IdentExpr {
        ident: super::ident(v),
    }
}

pub struct NewExprBuilder {
    local: bool,
    ty: ast::Ident,
    args: Vec<ast::PureExpr>,
}

impl NewExprBuilder {
    pub fn new<S: Into<String>>(local: bool, ty: S) -> Self {
        Self {
            local,
            ty: super::ident(ty),
            args: vec![],
        }
    }

    pub fn add_arg(&mut self, arg: ast::PureExpr) {
        self.args.push(arg)
    }

    pub fn with_args(mut self, arg: ast::PureExpr) -> Self {
        self.add_arg(arg);
        self
    }

    pub fn complete(self) -> ast::NewExpr {
        ast::NewExpr {
            local: self.local,
            ty: self.ty,
            args: self.args,
        }
    }
}

pub fn start_new_expr<S: Into<String>>(local: bool, ty: S) -> NewExprBuilder {
    NewExprBuilder::new(local, ty)
}

pub fn create_null() -> ast::PureExpr {
    ast::PureExpr::Null
}

pub fn create_bin_expr(
    op: ast::BinaryOp,
    left: ast::PureExpr,
    right: ast::PureExpr,
) -> ast::BinaryExpr {
    ast::BinaryExpr {
        op,
        left: left.into(),
        right: right.into(),
    }
}

pub fn create_ne_expr(left: ast::PureExpr, right: ast::PureExpr) -> ast::BinaryExpr {
    create_bin_expr(ast::BinaryOp::Ne, left, right)
}

pub fn create_data_constr<S: Into<String>>(name: S) -> ast::DataConstrExpr {
    ast::DataConstrExpr {
        ident: ident(name),
        args: Vec::new(),
    }
}

pub fn create_data_constr_args<S: Into<String>>(
    name: S,
    args: Vec<ast::PureExpr>,
) -> ast::DataConstrExpr {
    ast::DataConstrExpr {
        ident: ident(name),
        args,
    }
}
