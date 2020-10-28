use abs_syntax::ast;

pub struct BlockBuilder {
    stmts: Vec<ast::Stmt>,
}

impl BlockBuilder {
    pub fn new() -> Self {
        BlockBuilder { stmts: vec![] }
    }

    pub fn add_stmt(&mut self, stmt: ast::Stmt) {
        self.stmts.push(stmt)
    }

    pub fn with_stmt(mut self, stmt: ast::Stmt) -> Self {
        self.add_stmt(stmt);
        self
    }

    pub fn complete(self) -> ast::Block {
        ast::Block { stmts: self.stmts }
    }
}

pub fn start_block() -> BlockBuilder {
    BlockBuilder::new()
}

pub fn create_ret_stmt(expr: ast::Expr) -> ast::ReturnStmt {
    ast::ReturnStmt { expr }
}

pub fn create_var_decl<S: Into<String>>(
    ty: ast::Type,
    name: S,
    annotations: ast::Annotations,
) -> ast::VarDeclStmt {
    ast::VarDeclStmt {
        annotations,
        ty,
        ident: super::ident(name),
        init: None,
    }
}

pub fn create_var_decl_init<S: Into<String>>(
    ty: ast::Type,
    name: S,
    init: ast::Expr,
    annotations: ast::Annotations,
) -> ast::VarDeclStmt {
    ast::VarDeclStmt {
        annotations,
        ty,
        ident: super::ident(name),
        init: Some(init),
    }
}

pub fn create_assign<S: Into<String>>(name: S, expr: ast::Expr) -> ast::AssignStmt {
    ast::AssignStmt {
        this: false,
        ident: super::ident(name),
        expr,
    }
}

pub fn create_field_assign<S: Into<String>>(name: S, expr: ast::Expr) -> ast::AssignStmt {
    ast::AssignStmt {
        this: true,
        ident: super::ident(name),
        expr,
    }
}
