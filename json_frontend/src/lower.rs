use std::error::Error;

use serde_json::Value;

use midlang::middle_lang as m;

use crate::json_lang::*;

type Res<T> = Result<T, Box<dyn Error>>;

pub fn lower(json_lang: &JSONLang) -> Res<Vec<m::Module>> {
    let modules = match json_lang {
        JSONLang::Modules(modules) => {
            let mut lowered = Vec::<m::Module>::with_capacity(modules.len());

            for module in modules {
                lowered.push(m::Module {
                    name: module.name.to_string(),
                    decls: lower_decls(&module.decls)?,
                });
            }

            lowered
        }
    };

    Ok(modules)
}

fn lower_decls(decls: &[Decl]) -> Res<Vec<m::Decl>> {
    decls.iter().map(lower_decl).collect()
}

fn lower_decl(decl: &Decl) -> Res<m::Decl> {
    match decl {
        Decl::FwdDecl {
            name,
            visibility,
            r#type,
            args,
            variadic,
        } => Ok(m::Decl::FwdDecl(
            name.to_string(),
            lower_visibility(visibility),
            lower_type(r#type),
            lower_args(args),
            variadic.unwrap_or(false),
        )),
        Decl::FuncDecl {
            name,
            visibility,
            r#type,
            args,
            variadic,
            stmts,
        } => Ok(m::Decl::FuncDecl(
            name.to_string(),
            lower_visibility(visibility),
            lower_type(r#type),
            lower_args(args),
            variadic.unwrap_or(false),
            lower_stmts(stmts)?,
        )),
    }
}

fn lower_visibility(visibility: &Visibility) -> m::Visibility {
    match visibility {
        Visibility::Public => m::Visibility::Public,
        Visibility::Private => m::Visibility::Private,
    }
}

fn lower_type(r#type: &Type) -> m::Type {
    match r#type {
        Type::Int32 => m::Type::Int32,
        Type::Str => m::Type::Str,
    }
}

fn lower_args(args: &Vec<FuncArg>) -> Vec<m::FuncArg> {
    args.iter()
        .map(|a| (a.name.to_string(), lower_type(&a.r#type)))
        .collect()
}

fn lower_stmts(stmts: &[Stmt]) -> Res<Vec<m::Stmt>> {
    stmts.iter().map(lower_stmt).collect()
}

fn lower_stmt(stmt: &Stmt) -> Res<m::Stmt> {
    match stmt {
        Stmt::Ret { value } => Ok(m::Stmt::Ret(lower_expr(value)?)),
        Stmt::VarDecl { name, value } => Ok(m::Stmt::VarDecl(name.to_string(), lower_expr(value)?)),
    }
}

fn lower_exprs(exprs: &[Expr]) -> Res<Vec<m::Expr>> {
    exprs.iter().map(lower_expr).collect()
}

fn lower_expr(expr: &Expr) -> Res<m::Expr> {
    match expr {
        Expr::Const { value, r#type } => match (value, r#type) {
            (Value::String(s), Type::Str) => Ok(m::Expr::ConstStr(s.to_string())),
            (Value::Number(n), _) => Ok(lower_number(n, r#type)?),
            _ => Err(Box::from("Unsupported value and type")),
        },
        Expr::FuncCall { name, r#type, args } => Ok(m::Expr::FuncCall(
            name.to_string(),
            lower_type(r#type),
            lower_exprs(args)?,
        )),
    }
}

fn lower_number(num: &serde_json::value::Number, r#type: &Type) -> Res<m::Expr> {
    fn as_i32(num: &serde_json::value::Number) -> Res<i32> {
        num.as_i64()
            .map(i32::try_from)
            .and_then(|r| r.ok())
            .ok_or_else(|| Box::from("Number is not an Int32"))
    }

    match (num, r#type) {
        (n, Type::Int32) => Ok(m::Expr::ConstInt32(as_i32(n)?)),
        _ => Err(Box::from("Invalid number value and type")),
    }
}
