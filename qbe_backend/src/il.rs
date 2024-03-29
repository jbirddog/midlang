use std::fmt;
use std::fmt::Write;

use compiler::BuildArtifacts;

use crate::lower_lang::*;

const IL_BUFFER_CAPACITY: usize = 1024;
const INDENT: &str = "    ";

const RENDER_VALUE_PLAIN: u8 = 0;
const RENDER_VALUE_TYPES: u8 = 1 << 0;
const RENDER_VALUE_COPY_LITERALS: u8 = 1 << 1;

pub fn generate_il(comp_units: &[CompUnit]) -> Result<BuildArtifacts, fmt::Error> {
    let mut build_artifacts = BuildArtifacts::with_capacity(comp_units.len());

    for comp_unit in comp_units {
        build_artifacts.push((filename(&comp_unit.name), decls_il(&comp_unit.decls)?));
    }

    Ok(build_artifacts)
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
        Decl::FuncDecl(name, linkage, r#type, args, variadic, stmts) => {
            if let Some(linkage) = linkage {
                write!(il, "{} ", linkage)?;
            }

            il.write_str("function ")?;

            if let Some(r#type) = r#type {
                write!(il, "{} ", r#type)?;
            }

            write!(il, "${}(", name)?;

            append_func_args_il(args, *variadic, il)?;

            il.write_str(") {\n")?;

            append_stmts_il(stmts, il)?;

            il.write_str("}\n")?;
        }
    }

    Ok(())
}

fn append_func_args_il(args: &[FuncArg], variadic: bool, il: &mut impl Write) -> fmt::Result {
    for (i, (name, r#type)) in args.iter().enumerate() {
        if i > 0 {
            il.write_str(", ")?;
        }

        write!(il, "{} %{}", r#type, name)?;
    }

    if variadic {
        il.write_str(", ...")?;
    }

    Ok(())
}

fn append_func_call_il(
    name: &str,
    values: &[Value],
    indent: bool,
    il: &mut impl Write,
) -> fmt::Result {
    if indent {
        il.write_str(INDENT)?;
    }
    write!(il, "call ${}(", name)?;

    for (i, value) in values.iter().enumerate() {
        if i > 0 {
            il.write_str(", ")?;
        }

        append_value_il(value, RENDER_VALUE_TYPES, il)?;
    }

    il.write_str(")")?;

    Ok(())
}

fn append_stmts_il(stmts: &[Stmt], il: &mut impl Write) -> fmt::Result {
    for stmt in stmts {
        match stmt {
            Stmt::FuncCall(name, values) => append_func_call_il(name, values, true, il)?,
            Stmt::Jmp(lbl) => write!(il, "{}jmp @{}", INDENT, lbl)?,
            Stmt::Jnz(value, true_lbl, false_lbl) => {
                write!(il, "{}jnz ", INDENT)?;
                append_value_il(value, RENDER_VALUE_PLAIN, il)?;
                write!(il, ", @{}, @{}", true_lbl, false_lbl)?;
            }
            Stmt::Lbl(name) => write!(il, "@{}", name)?,
            Stmt::Ret(Some(value)) => {
                write!(il, "{}ret ", INDENT)?;
                append_value_il(value, RENDER_VALUE_PLAIN, il)?;
            }
            Stmt::Ret(None) => write!(il, "{}ret", INDENT)?,
            Stmt::Store(r#type, src, dest) => {
                write!(il, "{}store{} ", INDENT, r#type)?;
                append_value_il(src, RENDER_VALUE_PLAIN, il)?;
                il.write_str(", ")?;
                append_value_il(dest, RENDER_VALUE_PLAIN, il)?;
            }
            Stmt::VarDecl(name, scope, expr) => {
                write!(il, "{}{}{} ={} ", INDENT, scope, name, expr.r#type())?;
                append_expr_il(expr, RENDER_VALUE_COPY_LITERALS, il)?;
            }
        }

        il.write_str("\n")?;
    }

    Ok(())
}

fn append_expr_il(expr: &Expr, value_render_flags: u8, il: &mut impl Write) -> fmt::Result {
    match expr {
        Expr::Alloc8(bytes) => write!(il, "alloc8 {}", bytes)?,
        Expr::Cmp(op, lhs, rhs) => {
            write!(il, "c{}{} ", op, lhs.r#type())?;
            append_value_il(lhs, RENDER_VALUE_PLAIN, il)?;
            il.write_str(", ")?;
            append_value_il(rhs, RENDER_VALUE_PLAIN, il)?;
        }
        Expr::Load(_, r#type, value) => {
            write!(il, "load{} ", r#type)?;
            append_value_il(value, value_render_flags, il)?;
        }
        Expr::Sub(value1, value2) => {
            il.write_str("sub ")?;
            append_value_il(value1, RENDER_VALUE_PLAIN, il)?;
            il.write_str(", ")?;
            append_value_il(value2, RENDER_VALUE_PLAIN, il)?;
        }
        Expr::Value(value) => append_value_il(value, value_render_flags, il)?,
        Expr::FuncCall(name, _, values) => append_func_call_il(name, values, false, il)?,
    }

    Ok(())
}

fn append_value_il(value: &Value, render_flags: u8, il: &mut impl Write) -> fmt::Result {
    if render_flags & RENDER_VALUE_TYPES != 0 {
        write!(il, "{} ", value.r#type())?;
    }

    match value {
        Value::ConstD(v) => {
            if render_flags & RENDER_VALUE_COPY_LITERALS != 0 {
                il.write_str("copy ")?;
            }

            write!(il, "d_{}", v)?;
        }
        Value::ConstL(v) => {
            if render_flags & RENDER_VALUE_COPY_LITERALS != 0 {
                il.write_str("copy ")?;
            }

            write!(il, "{}", v)?;
        }
        Value::ConstW(v) => {
            if render_flags & RENDER_VALUE_COPY_LITERALS != 0 {
                il.write_str("copy ")?;
            }

            write!(il, "{}", v)?;
        }
        Value::VarRef(name, _, scope) => write!(il, "{}{}", scope, name)?,
    }

    Ok(())
}
