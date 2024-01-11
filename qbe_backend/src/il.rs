use std::fmt;
use std::fmt::Write;

use midlang::compiler::BuildArtifacts;

use crate::lower_lang::*;

const IL_BUFFER_CAPACITY: usize = 1024;
const INDENT: &str = "    ";

pub fn generate_il(lower_lang: &LowerLang) -> Result<BuildArtifacts, fmt::Error> {
    match lower_lang {
        LowerLang::CompUnit(name, decls) => Ok(vec![(filename(name), decls_il(decls)?)]),
    }
}

fn filename(name: &str) -> String {
    format!("{}.il", name)
}

fn decls_il(decls: &[Decl]) -> Result<String, fmt::Error> {
    let mut il = String::with_capacity(IL_BUFFER_CAPACITY);

    for decl in decls {
        append_decl_il(decl, &mut il)?;
    }

    Ok(il)
}

fn append_decl_il(decl: &Decl, il: &mut impl Write) -> fmt::Result {
    match decl {
        Decl::Data(name, fields) => {
            write!(il, "data ${} = {{ ", name)?;

            for (i, (r#type, value)) in fields.iter().enumerate() {
                if i > 0 {
                    il.write_str(", ")?;
                }

                write!(il, "{} {}", r#type, value)?;
            }

            il.write_str(" }\n")?;
        }
        Decl::FuncDecl(name, linkage, r#type, args, stmts) => {
            if let Some(linkage) = linkage {
                write!(il, "{} ", linkage)?;
            }

            write!(il, "function {} ${}(", r#type, name)?;

            append_func_args_il(args, il)?;

            il.write_str(") {\n")?;

            append_stmts_il(stmts, il)?;

            il.write_str("}\n")?;
        }
    }

    Ok(())
}

fn append_func_args_il(args: &FuncArgs, il: &mut impl Write) -> fmt::Result {
    match args {
        FuncArgs::Fixed(args) => {
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    il.write_str(", ")?;
                }
                append_func_arg_il(arg, il)?;
            }
        }
        FuncArgs::Variadic(first, rest) => {
            append_func_arg_il(first, il)?;

            for arg in rest {
                il.write_str(", ")?;
                append_func_arg_il(arg, il)?;
            }

            il.write_str(", ...")?;
        }
    }

    Ok(())
}

fn append_func_arg_il(arg: &FuncArg, il: &mut impl Write) -> fmt::Result {
    match arg {
        FuncArg::Named(name, r#type) => write!(il, "{} %{}", r#type, name)?,
    }

    Ok(())
}

fn append_stmts_il(stmts: &[Stmt], il: &mut impl Write) -> fmt::Result {
    for stmt in stmts {
        match stmt {
            Stmt::Lbl(name) => write!(il, "@{}", name)?,
            Stmt::Ret(value) => {
                write!(il, "{}ret ", INDENT)?;
                append_value_il(value, false, il)?;
            }
            Stmt::VarDecl(name, scope, expr) => {
                write!(il, "{}{}{} ={} ", INDENT, scope, name, expr.r#type())?;
                append_expr_il(expr, false, il)?;
            }
        }

        il.write_str("\n")?;
    }

    Ok(())
}

fn append_expr_il(expr: &Expr, type_consts: bool, il: &mut impl Write) -> fmt::Result {
    match expr {
        Expr::Value(value) => append_value_il(value, type_consts, il)?,
        Expr::FuncCall(name, _, values) => {
            write!(il, "call ${}(", name)?;

            for (i, value) in values.iter().enumerate() {
                if i > 0 {
                    il.write_str(", ")?;
                }

                append_value_il(value, true, il)?;
            }

            il.write_str(")")?;
        }
    }

    Ok(())
}

fn append_value_il(value: &Value, type_consts: bool, il: &mut impl Write) -> fmt::Result {
    match value {
        Value::ConstW(v) => {
            if type_consts {
                write!(il, "{} ", Type::W)?;
            }
            write!(il, "{}", v)?;
        }
        Value::VarRef(name, r#type, scope) => write!(il, "{} {}{}", r#type, scope, name)?,
    }

    Ok(())
}