use std::io::prelude::*;
use std::{
    fs::{self, File},
    io,
    path::Path,
};

use abs_syntax::ast;
use gen::ty;

mod gen;

fn gen_mock_module() -> ast::Module {
    gen::start_module("MockABS")
        .with_child(interface_i())
        .with_child(interface_j())
        .with_child(class_d())
        .with_child(class_e())
        .with_child(class_generated())
        .complete()
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

fn interface_j() -> ast::InterfaceDecl {
    gen::start_interface_decl("J")
        .with_sig(
            gen::start_method_sig("m")
                .with_ret(ty::create_unit())
                .with_param(gen::create_param(ty::create_int(), "v"))
                .complete(),
        )
        .with_sig(
            gen::start_method_sig("getI")
                .with_ret(ty::simple_ty("I"))
                .with_param(gen::create_param(ty::create_bool(), "flag"))
                .with_param(gen::create_param(ty::create_int(), "c"))
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

fn class_e() -> ast::ClassDecl {
    gen::start_class_decl("E")
        .with_implements("J")
        .with_method(gen::create_method_decl(
            gen::start_method_sig("m")
                .with_ret(ty::create_unit())
                .with_param(gen::create_param(ty::create_int(), "v"))
                .complete(),
            gen::start_block().complete(),
        ))
        .with_method(gen::create_method_decl(
            gen::start_method_sig("getI")
                .with_ret(ty::simple_ty("I"))
                .with_param(gen::create_param(ty::create_bool(), "flag"))
                .with_param(gen::create_param(ty::create_int(), "c"))
                .complete(),
            gen::start_block()
                .with_stmt(
                    gen::create_var_decl_init(
                        ty::simple_ty("I"),
                        "res",
                        ast::EffExpr::New(gen::start_new_expr(false, "D").complete()).into(),
                    )
                    .into(),
                )
                .with_stmt(gen::create_ret_stmt(gen::create_var_use("res").into()).into())
                .complete(),
        ))
        .complete()
}

fn class_generated() -> ast::ClassDecl {
    gen::start_class_decl("Generated")
        .with_field(gen::create_field_init(
            ty::create_int(),
            "fint",
            gen::create_lit("0").into(),
        ))
        .with_field(gen::create_field_init(
            ty::create_bool(),
            "fb",
            gen::create_lit("True").into(),
        ))
        .with_field(gen::create_field(ty::create_fut(ty::create_int()), "ff"))
        .with_field(gen::create_field(ty::create_fut(ty::create_int()), "ffb"))
        .with_field(gen::create_field_init(
            ty::simple_ty("I"),
            "fi",
            gen::create_null(),
        ))
        .with_field(gen::create_field_init(
            ty::simple_ty("J"),
            "fj",
            gen::create_null(),
        ))
        .complete()
}

fn clear_out() -> io::Result<()> {
    let path = Path::new("./out/");
    for e in fs::read_dir(path)? {
        fs::remove_file(e?.path())?;
    }

    Ok(())
}

fn write_module(idx: u32) -> io::Result<()> {
    let module = gen_mock_module();

    let path_str = format!("./out/mod-{}.abs", idx);
    let path = Path::new(&path_str);

    let mut f = File::create(path)?;

    f.write_all(module.to_string().as_bytes())?;

    Ok(())
}

const NUM_MODULES: u32 = 100;

fn main() {
    clear_out().expect("Err while clearing out dir");
    for i in 0..NUM_MODULES {
        write_module(i).expect("An error occurred while writing the module");
    }
}
