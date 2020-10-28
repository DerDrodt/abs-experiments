use std::io::prelude::*;
use std::{
    fs::{self, File},
    io,
    path::Path,
};

use abs_syntax::ast;
use gen::ty;

mod gen;

#[derive(Debug, Copy, Clone)]
pub enum Target {
    Crowbar,
    NullableExtension,
}

fn gen_mock_module(number_of_rand_classes: u32, target: Target) -> ast::Module {
    let mut builder = gen::start_module("MockABS");

    if let Target::Crowbar = target {
        builder.add_child(
            gen::start_data_type("Spec")
                .with_const(
                    gen::start_data_constr("ObjInv")
                        .with_param(gen::create_data_constr_param(ty::create_bool()))
                        .complete(),
                )
                .with_const(
                    gen::start_data_constr("Ensures")
                        .with_param(gen::create_data_constr_param(ty::create_bool()))
                        .complete(),
                )
                .with_const(
                    gen::start_data_constr("Requires")
                        .with_param(gen::create_data_constr_param(ty::create_bool()))
                        .complete(),
                )
                .with_const(
                    gen::start_data_constr("WhileInv")
                        .with_param(gen::create_data_constr_param(ty::create_bool()))
                        .complete(),
                )
                .complete(),
        )
    }

    let mut builder = builder
        .with_child(interface_i())
        .with_child(interface_j(target))
        .with_child(class_d())
        .with_child(class_e(target));

    for i in 0..number_of_rand_classes {
        let name = format!("Generated_{}", i);
        builder.add_child(class_generated(&name, target));
    }

    builder.complete()
}

fn interface_i() -> ast::InterfaceDecl {
    gen::start_interface_decl("I")
        .with_sig(
            gen::start_method_sig("n")
                .with_ret(ty::create_int())
                .complete(),
        )
        .with_sig(
            gen::start_method_sig("b")
                .with_ret(ty::create_bool())
                .complete(),
        )
        .complete()
}

fn interface_j(target: Target) -> ast::InterfaceDecl {
    gen::start_interface_decl("J")
        .with_sig(
            gen::start_method_sig("m")
                .with_ret(ty::create_unit())
                .with_param(gen::create_param(ty::create_int(), "v", gen::empty_annos()))
                .complete(),
        )
        .with_sig(
            gen::start_method_sig("getI")
                .with_annotation(gen::create_non_null_ret_anno(target))
                .with_ret(ty::simple_ty("I"))
                .with_param(gen::create_param(
                    ty::create_bool(),
                    "flag",
                    gen::empty_annos(),
                ))
                .with_param(gen::create_param(ty::create_int(), "c", gen::empty_annos()))
                .complete(),
        )
        .complete()
}

fn class_d() -> ast::ClassDecl {
    gen::start_class_decl("D")
        .with_implements("I")
        .with_method(gen::create_method_decl(
            gen::start_method_sig("n")
                .with_ret(ty::create_int())
                .complete(),
            gen::start_block()
                .with_stmt(gen::create_ret_stmt(gen::create_lit("0").into()).into())
                .complete(),
        ))
        .with_method(gen::create_method_decl(
            gen::start_method_sig("b")
                .with_ret(ty::create_bool())
                .complete(),
            gen::start_block()
                .with_stmt(gen::create_ret_stmt(gen::create_lit("False").into()).into())
                .complete(),
        ))
        .complete()
}

fn class_e(target: Target) -> ast::ClassDecl {
    let mut get_i_sig = gen::start_method_sig("getI")
        .with_ret(ty::simple_ty("I"))
        .with_param(gen::create_param(
            ty::create_bool(),
            "flag",
            gen::empty_annos(),
        ))
        .with_param(gen::create_param(ty::create_int(), "c", gen::empty_annos()));

    if let Target::NullableExtension = target {
        get_i_sig.add_annotation(gen::create_nullable_non_null())
    }

    let builder = gen::start_class_decl("E")
        .with_implements("J")
        .with_method(gen::create_method_decl(
            gen::start_method_sig("m")
                .with_ret(ty::create_unit())
                .with_param(gen::create_param(ty::create_int(), "v", gen::empty_annos()))
                .complete(),
            gen::start_block().complete(),
        ))
        .with_method(gen::create_method_decl(
            get_i_sig.complete(),
            gen::start_block()
                .with_stmt(
                    gen::create_var_decl_init(
                        ty::simple_ty("I"),
                        "res",
                        ast::EffExpr::New(gen::start_new_expr(false, "D").complete()).into(),
                        gen::empty_annos(),
                    )
                    .into(),
                )
                .with_stmt(gen::create_ret_stmt(gen::create_var_use("res").into()).into())
                .complete(),
        ));
    builder.complete()
}

fn class_generated(name: &str, target: Target) -> ast::ClassDecl {
    let mut builder = gen::start_class_decl(name)
        .with_field(gen::create_field_init(
            ty::create_int(),
            "fint",
            gen::create_lit("0").into(),
            gen::empty_annos(),
        ))
        .with_field(gen::create_field_init(
            ty::create_bool(),
            "fb",
            gen::create_lit("True").into(),
            gen::empty_annos(),
        ))
        .with_field(gen::create_field(
            ty::create_fut(ty::create_int()),
            "ff",
            gen::empty_annos(),
        ))
        .with_field(gen::create_field(
            ty::create_fut(ty::create_int()),
            "ffb",
            gen::empty_annos(),
        ))
        .with_field(gen::create_field_init(
            ty::simple_ty("I"),
            "fi",
            gen::create_null(),
            gen::empty_annos(),
        ))
        .with_field(gen::create_field_init(
            ty::simple_ty("J"),
            "fj",
            gen::create_null(),
            gen::empty_annos(),
        ));

    builder.add_method(create_rand_method(target));

    builder.complete()
}

fn create_rand_method(target: Target) -> ast::MethodDecl {
    let mut sig = gen::start_method_sig("gen")
        .with_ret(ty::simple_ty("I"))
        .with_annotation(gen::create_non_null_ret_anno(target));

    let param = match target {
        Target::Crowbar => {
            sig.add_annotation(gen::create_crowbar_non_null_param("i"));
            gen::create_param(ty::simple_ty("I"), "i", gen::empty_annos())
        }
        Target::NullableExtension => {
            let mut annos = gen::empty_annos();
            annos.push(gen::create_nullable_non_null());
            gen::create_param(ty::simple_ty("I"), "i", annos)
        }
    };

    sig.add_param(param);

    gen::create_method_decl(sig.complete(), gen::start_block().complete())
}

fn clear_out() -> io::Result<()> {
    let path = Path::new("./out/");
    for e in fs::read_dir(path)? {
        fs::remove_file(e?.path())?;
    }

    Ok(())
}

fn write_module(number_of_rand_classes: u32) -> io::Result<()> {
    let module = gen_mock_module(number_of_rand_classes, Target::NullableExtension);

    let path_str = "./out/generated.abs";
    let path = Path::new(path_str);

    let mut f = File::create(path)?;

    f.write_all(module.to_string().as_bytes())?;

    Ok(())
}

const NUM_RAND_CLASSES: u32 = 100;

fn main() {
    clear_out().expect("Err while clearing out dir");

    write_module(NUM_RAND_CLASSES).expect("An error occurred while writing the module");
}
