use abs_syntax::ast;

use crate::{gen, Target};

pub fn empty_annos() -> ast::Annotations {
    ast::Annotations::default()
}

pub fn create_untyped_anno(e: ast::PureExpr) -> ast::Annotation {
    ast::Annotation::Untyped(ast::UntypedAnnotation(e))
}

pub fn create_typed_anno(ty: ast::Type, expr: ast::PureExpr) -> ast::Annotation {
    ast::Annotation::Typed(ast::TypedAnnotation { ty, expr })
}

pub fn create_nullable_nullable() -> ast::Annotation {
    create_untyped_anno(gen::create_data_constr("Nullable").into())
}

pub fn create_nullable_non_null() -> ast::Annotation {
    create_untyped_anno(gen::create_data_constr("NonNull").into())
}

pub fn create_crowbar_non_null_param<S: Into<String>>(var: S) -> ast::Annotation {
    create_typed_anno(
        gen::ty::simple_ty("Spec"),
        gen::create_data_constr_args(
            "Requires",
            vec![
                gen::create_ne_expr(gen::create_var_use(var).into(), gen::create_null().into())
                    .into(),
            ],
        )
        .into(),
    )
}

pub fn create_non_null_ret_anno(target: Target) -> ast::Annotation {
    match target {
        Target::Crowbar => create_typed_anno(
            gen::ty::simple_ty("Spec"),
            gen::create_data_constr_args(
                "Ensures",
                vec![gen::create_ne_expr(
                    gen::create_var_use("result").into(),
                    gen::create_null().into(),
                )
                .into()],
            )
            .into(),
        ),
        Target::NullableExtension => create_nullable_non_null(),
    }
}
