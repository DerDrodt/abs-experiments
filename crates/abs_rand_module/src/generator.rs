use rand::seq::SliceRandom;

use abs_syntax::ast;

use crate::{chance, gen, Options, Target};

pub fn ty_is_obj(ty: &ast::Type) -> bool {
    let str = &ty.ident.str;
    str == "I" || str == "J"
}

pub struct RandGenerator {
    scope: Scope,
    opts: Options,
    has_null_check_if: bool,
}

impl RandGenerator {
    pub fn new(opts: Options) -> Self {
        Self {
            scope: Scope::new(),
            opts,
            has_null_check_if: false,
        }
    }

    pub fn generate_body(&mut self) -> ast::Block {
        let size = chance::exp_rand_int(self.opts.avg_meth_body_size as f64);

        self.scope
            .define_field(gen::ty::create_int(), gen::ident("fint"));
        self.scope
            .define_field(gen::ty::create_bool(), gen::ident("fb"));
        self.scope
            .define_field(gen::ty::create_fut(gen::ty::create_int()), gen::ident("ff"));
        self.scope.define_field(
            gen::ty::create_fut(gen::ty::create_bool()),
            gen::ident("ffb"),
        );
        self.scope
            .define_field(gen::ty::simple_ty("I"), gen::ident("fi"));
        self.scope
            .define_field(gen::ty::simple_ty("J"), gen::ident("fj"));

        self.scope.define_fn(
            gen::ty::create_int(),
            gen::ident("n"),
            vec![gen::ty::simple_ty("I")],
            vec![],
        );
        self.scope.define_fn(
            gen::ty::create_bool(),
            gen::ident("b"),
            vec![gen::ty::simple_ty("I")],
            vec![],
        );
        self.scope.define_fn(
            gen::ty::create_unit(),
            gen::ident("m"),
            vec![gen::ty::simple_ty("J")],
            vec![gen::ty::create_int()],
        );
        self.scope.define_fn(
            gen::ty::simple_ty("I"),
            gen::ident("getI"),
            vec![gen::ty::simple_ty("J")],
            vec![gen::ty::create_bool(), gen::ty::create_int()],
        );

        let mut builder = self.generate_sized_block(size);

        if !self.has_null_check_if {
            builder.add_stmt(self.generate_null_check_if());
        }

        builder.add_stmt(self.generate_ret());

        builder.complete()
    }

    pub fn generate_block(&mut self) -> ast::Block {
        let size = chance::exp_rand_int(self.opts.avg_block_size as f64);

        self.generate_sized_block(size).complete()
    }

    pub fn generate_sized_block(&mut self, size: u64) -> gen::BlockBuilder {
        self.scope.open();
        let mut builder = gen::start_block();

        for _ in 0..size {
            builder.add_stmt(self.generate_stmt())
        }
        self.scope.close();
        builder
    }

    pub fn generate_stmt(&mut self) -> ast::Stmt {
        if self.scope.depth() < self.opts.max_depth as usize
            && chance::chance(self.opts.branch_rate)
        {
            if !self.has_null_check_if && self.scope.depth() == 0 && chance::chance(0.3) {
                self.generate_null_check_if()
            } else {
                self.generate_if()
            }
        } else if chance::chance(self.opts.declare_to_assign) {
            self.generate_decl()
        } else {
            self.generate_assign()
        }
    }

    fn generate_ret(&mut self) -> ast::Stmt {
        let expr = ast::PureExpr::Ident(ast::IdentExpr {
            ident: gen::ident("i"),
        })
        .into();
        ast::ReturnStmt { expr }.into()
    }

    pub fn generate_if(&mut self) -> ast::Stmt {
        let condition = self.generate_pure_exp(gen::ty::create_bool());

        let then = Box::new(self.generate_block().into());

        let r#else = if chance::chance(self.opts.else_ratio) {
            Some(Box::new(self.generate_block().into()))
        } else {
            None
        };

        ast::IfStmt {
            condition,
            then,
            r#else,
        }
        .into()
    }

    pub fn generate_null_check_if(&mut self) -> ast::Stmt {
        self.has_null_check_if = true;

        let condition = ast::BinaryExpr {
            op: ast::BinaryOp::Eq,
            left: Box::new(
                ast::IdentExpr {
                    ident: gen::ident("i"),
                }
                .into(),
            ),
            right: gen::create_null().into(),
        }
        .into();

        let then = self.generate_block().into();

        let then = match then {
            ast::Stmt::Block(mut b) => {
                let expr: ast::EffExpr = ast::NewExpr {
                    ty: gen::ident("D"),
                    args: vec![],
                    local: false,
                }
                .into();
                b.stmts.push(
                    ast::AssignStmt {
                        ident: gen::ident("i"),
                        expr: expr.into(),
                        this: false,
                    }
                    .into(),
                );
                ast::Stmt::Block(b)
            }
            _ => then,
        };

        let r#else = if chance::chance(self.opts.else_ratio) {
            Some(Box::new(self.generate_block().into()))
        } else {
            None
        };

        ast::IfStmt {
            condition,
            then: Box::new(then),
            r#else,
        }
        .into()
    }

    pub fn generate_decl(&mut self) -> ast::Stmt {
        let ty = self.rand_avail_ty(true);
        let ident = self.scope.free_var_ident(ty.clone());
        let init = self.generate_expr(ty.clone());

        self.scope.define_var(ty.clone(), ident.clone());

        ast::VarDeclStmt {
            annotations: ast::Annotations::default(),
            ty,
            ident,
            init: Some(init),
        }
        .into()
    }

    pub fn generate_assign(&mut self) -> ast::Stmt {
        let ScopeEntry { ident, ty, .. } = self.scope.get_assignable_ident();
        let expr = self.generate_expr(ty.clone());
        ast::AssignStmt {
            this: false,
            ident,
            expr,
        }
        .into()
    }

    fn generate_expr(&mut self, ty: ast::Type) -> ast::Expr {
        let fut_ty = gen::ty::create_fut(ty.clone());
        let e: ast::EffExpr =
            if !ty.is_fut() && self.scope.has_of_type(fut_ty.clone()) && chance::chance(0.1) {
                ast::GetExpr {
                    expr: self.generate_pure_exp(fut_ty),
                }
                .into()
            } else if ty_is_obj(&ty) && chance::chance(0.5) {
                let ident = if ty.ident.str == "I" { "D" } else { "E" };
                let ident = gen::ident(ident);
                ast::NewExpr {
                    ty: ident,
                    args: vec![],
                    local: false,
                }
                .into()
            } else if ty.is_fut()
                && chance::chance(0.7)
                && self.scope.fn_of_type(ty.args[0].clone()).count() > 0
            {
                let f = self
                    .scope
                    .fn_of_type(ty.args[0].clone())
                    .collect::<Vec<_>>()
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone();
                let callee = f
                    .defined_for
                    .choose(&mut rand::thread_rng())
                    .expect(&f.ident.str)
                    .clone();
                let callee = self.generate_pure_exp(callee);
                let args = f
                    .args
                    .iter()
                    .map(|a| self.generate_pure_exp(a.clone()))
                    .collect();
                ast::AsyncCallExpr {
                    callee,
                    args,
                    method: f.ident,
                }
                .into()
            } else if self.opts.target != Target::Location
                && chance::chance(0.1)
                && self.scope.fn_of_type(ty.clone()).count() > 0
            {
                let f = self
                    .scope
                    .fn_of_type(ty.clone())
                    .collect::<Vec<_>>()
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone();
                let callee = f
                    .defined_for
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone();
                let callee = self.generate_pure_exp(callee);
                let args = f
                    .args
                    .iter()
                    .map(|a| self.generate_pure_exp(a.clone()))
                    .collect();
                ast::SyncCallExpr {
                    callee,
                    args,
                    method: f.ident,
                }
                .into()
            } else {
                return self.generate_pure_exp(ty).into();
            };
        e.into()
    }

    fn generate_pure_exp(&mut self, ty: ast::Type) -> ast::PureExpr {
        if ty.is_bool() && chance::chance(0.15) {
            let ty = gen::ty::create_int();
            let op = [
                ast::BinaryOp::Eq,
                ast::BinaryOp::Le,
                ast::BinaryOp::Ge,
                ast::BinaryOp::Ne,
                ast::BinaryOp::Gt,
                ast::BinaryOp::Lt,
            ]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone();
            return ast::BinaryExpr {
                op,
                left: self.generate_or(ty.clone()).into(),
                right: self.generate_or(ty).into(),
            }
            .into();
        } else if ty.is_bool() && chance::chance(0.15) {
            let ty = self.rand_avail_ty(true);
            return ast::BinaryExpr {
                op: ast::BinaryOp::Eq,
                left: self.generate_or(ty.clone()).into(),
                right: self.generate_or(ty).into(),
            }
            .into();
        } else {
            self.generate_or(ty)
        }
    }

    fn generate_or(&mut self, ty: ast::Type) -> ast::PureExpr {
        if ty.is_bool() && chance::chance(0.2) {
            ast::BinaryExpr {
                op: ast::BinaryOp::Eq,
                left: self.generate_and(ty.clone()).into(),
                right: self.generate_or(ty).into(),
            }
            .into()
        } else {
            self.generate_and(ty)
        }
    }

    fn generate_and(&mut self, ty: ast::Type) -> ast::PureExpr {
        if ty.is_bool() && chance::chance(0.2) {
            ast::BinaryExpr {
                op: ast::BinaryOp::Eq,
                left: self.generate_not(ty.clone()).into(),
                right: self.generate_and(ty).into(),
            }
            .into()
        } else {
            self.generate_not(ty)
        }
    }

    fn generate_not(&mut self, ty: ast::Type) -> ast::PureExpr {
        if ty.is_bool() && chance::chance(0.2) {
            ast::UnaryExpr {
                op: ast::UnaryOp::Not,
                expr: self.generate_add_sub(ty).into(),
            }
            .into()
        } else {
            self.generate_add_sub(ty)
        }
    }

    fn generate_add_sub(&mut self, ty: ast::Type) -> ast::PureExpr {
        if ty.is_int() && chance::chance(0.2) {
            let op = [ast::BinaryOp::Plus, ast::BinaryOp::Minus]
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone();
            ast::BinaryExpr {
                op,
                left: self.generate_add_sub(ty.clone()).into(),
                right: self.generate_mul_div(ty).into(),
            }
            .into()
        } else {
            self.generate_mul_div(ty)
        }
    }

    fn generate_mul_div(&mut self, ty: ast::Type) -> ast::PureExpr {
        if ty.is_int() && chance::chance(0.2) {
            let op = ast::BinaryOp::Mult;
            ast::BinaryExpr {
                op,
                left: self.generate_mul_div(ty.clone()).into(),
                right: self.generate_unary_minus(ty).into(),
            }
            .into()
        } else {
            self.generate_unary_minus(ty)
        }
    }

    fn generate_unary_minus(&mut self, ty: ast::Type) -> ast::PureExpr {
        if ty.is_int() && chance::chance(0.0) {
            ast::UnaryExpr {
                op: ast::UnaryOp::Minus,
                expr: self.generate_atom(ty).into(),
            }
            .into()
        } else {
            self.generate_atom(ty)
        }
    }

    fn generate_atom(&mut self, ty: ast::Type) -> ast::PureExpr {
        if ty.has_lit() && chance::chance(0.7) {
            ty.get_lit().unwrap().into()
        } else {
            let avail_idents = self.scope.of_type(ty.clone()).collect::<Vec<_>>();
            if avail_idents.is_empty() {
                panic!("Found no suitable ident for type {}", ty);
            }
            let ident = avail_idents
                .choose(&mut rand::thread_rng())
                .unwrap()
                .clone()
                .ident;
            ast::PureExpr::Ident(ast::IdentExpr { ident })
        }
    }

    fn rand_avail_ty(&self, allow_fut: bool) -> ast::Type {
        let mut ty;

        loop {
            let t = self.rand_ty(allow_fut);
            ty = Some(t.clone());
            if t.has_lit() || self.scope.has_of_type(t) {
                break;
            }
        }

        ty.unwrap()
    }

    fn rand_ty(&self, allow_fut: bool) -> ast::Type {
        if !allow_fut || chance::chance(0.8) {
            [
                gen::ty::create_int(),
                gen::ty::create_int(),
                gen::ty::create_bool(),
                gen::ty::create_bool(),
                gen::ty::create_i(),
                gen::ty::create_j(),
            ]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
        } else {
            gen::ty::create_fut(self.rand_ty(false))
        }
    }
}

struct Scope {
    stack: Vec<Vec<ScopeEntry>>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            stack: vec![Vec::new()],
        }
    }

    pub fn depth(&self) -> usize {
        self.stack.len() - 1
    }

    pub fn open(&mut self) {
        self.stack.push(Vec::new())
    }

    pub fn close(&mut self) {
        self.stack.pop();
    }

    pub fn define(&mut self, entry: ScopeEntry) {
        self.stack.last_mut().unwrap().push(entry)
    }

    pub fn define_field(&mut self, ty: ast::Type, ident: ast::Ident) {
        self.define(ScopeEntry::field(ty, ident))
    }

    pub fn define_var(&mut self, ty: ast::Type, ident: ast::Ident) {
        self.define(ScopeEntry::var(ty, ident))
    }

    pub fn define_fn(
        &mut self,
        ty: ast::Type,
        ident: ast::Ident,
        defined_for: Vec<ast::Type>,
        args: Vec<ast::Type>,
    ) {
        self.define(ScopeEntry::function(ty, ident, defined_for, args))
    }

    pub fn iter(&self) -> impl Iterator<Item = ScopeEntry> {
        let mut flattened = Vec::new();

        for level in self.stack.iter().rev() {
            for e in level {
                flattened.push(e.clone())
            }
        }

        flattened.into_iter()
    }

    pub fn functions(&self) -> impl Iterator<Item = ScopeEntry> {
        self.iter().filter(|e| e.kind == EntryKind::Fn)
    }

    pub fn of_type(&self, ty: ast::Type) -> impl Iterator<Item = ScopeEntry> {
        let name = &ty.ident.str;
        let is_fut = ty.is_fut();
        let arg0 = ty.args.get(0).map(|t| &t.ident.str);

        let v: Vec<ScopeEntry> = self
            .iter()
            .filter(move |e| {
                e.kind != EntryKind::Fn
                    && e.is_of_ty(name)
                    && (!is_fut || &e.ty.args[0].ident.str == arg0.unwrap())
            })
            .collect();
        v.into_iter()
    }

    pub fn has_of_type(&self, ty: ast::Type) -> bool {
        self.of_type(ty).next().is_some()
    }

    pub fn fn_of_type(&self, ty: ast::Type) -> impl Iterator<Item = ScopeEntry> {
        let name = &ty.ident.str;
        let v: Vec<ScopeEntry> = self.functions().filter(move |e| e.is_of_ty(name)).collect();
        v.into_iter()
    }

    fn sample(&self, i: impl Iterator<Item = ScopeEntry>) -> ScopeEntry {
        let v: Vec<ScopeEntry> = i.collect();

        v.choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn get_fut(&self) -> ScopeEntry {
        self.sample(
            self.iter()
                .filter(|e| e.kind != EntryKind::Fn && e.ty.is_fut()),
        )
    }

    pub fn get_fn(&self) -> ScopeEntry {
        self.sample(self.functions())
    }

    pub fn get_assignable_ident(&self) -> ScopeEntry {
        self.sample(self.iter().filter(|e| e.kind != EntryKind::Fn))
    }

    pub fn free_var_ident(&self, _ty: ast::Type) -> ast::Ident {
        let mut name = generate_name();

        while self.iter().any(|e| e.ident.str == name) {
            name = generate_name();
        }

        crate::gen::ident(name)
    }
}

#[derive(Clone)]
struct ScopeEntry {
    kind: EntryKind,
    ty: ast::Type,
    ident: ast::Ident,
    defined_for: Vec<ast::Type>,
    args: Vec<ast::Type>,
}

impl ScopeEntry {
    fn new(
        kind: EntryKind,
        ty: ast::Type,
        ident: ast::Ident,
        defined_for: Vec<ast::Type>,
        args: Vec<ast::Type>,
    ) -> Self {
        Self {
            kind,
            ty,
            ident,
            defined_for,
            args,
        }
    }

    fn field(ty: ast::Type, ident: ast::Ident) -> Self {
        Self::new(EntryKind::Field, ty, ident, Vec::new(), Vec::new())
    }

    fn var(ty: ast::Type, ident: ast::Ident) -> Self {
        Self::new(EntryKind::Var, ty, ident, Vec::new(), Vec::new())
    }

    fn function(
        ty: ast::Type,
        ident: ast::Ident,
        defined_for: Vec<ast::Type>,
        args: Vec<ast::Type>,
    ) -> Self {
        Self::new(EntryKind::Fn, ty, ident, defined_for, args)
    }

    fn is_of_ty(&self, name: &str) -> bool {
        self.ty.ident.str == name
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum EntryKind {
    Field,
    Var,
    Fn,
}

fn rand_char(i: u32) -> char {
    let mut range = if i == 0 || chance::chance(0.5) {
        'a'..='z'
    } else {
        'A'..='Z'
    };
    let idx = (rand::random::<f64>() * 26.0) as usize;

    range.nth(idx).unwrap()
}

fn generate_name() -> String {
    (0..).take(6).map(|i| rand_char(i)).collect()
}
