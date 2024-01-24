use std::error::Error;

use serde_json::Value;

use midlang as m;

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
            lower_opt_type(r#type),
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
            lower_opt_type(r#type),
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
        Type::Bool => m::Type::Bool,
        Type::Double => m::Type::Double,
        Type::Int32 => m::Type::Int32,
        Type::Int64 => m::Type::Int64,
        Type::Ptr => m::Type::Ptr,
        Type::Str => m::Type::Str,
    }
}

fn lower_opt_type(r#type: &Option<Type>) -> Option<m::Type> {
    r#type.as_ref().map(lower_type)
}

fn lower_args(args: &[FuncArg]) -> Vec<m::FuncArg> {
    args.iter()
        .map(|a| (a.name.to_string(), lower_type(&a.r#type)))
        .collect()
}

fn lower_stmts(stmts: &[Stmt]) -> Res<Vec<m::Stmt>> {
    stmts.iter().map(lower_stmt).collect()
}

fn lower_stmt(stmt: &Stmt) -> Res<m::Stmt> {
    match stmt {
        Stmt::Cond { cases } => Ok(m::Stmt::Cond(lower_cases(cases)?)),
        Stmt::FuncCall { name, args } => {
            Ok(m::Stmt::FuncCall(name.to_string(), lower_exprs(args)?))
        }
        Stmt::Ret { value: Some(value) } => Ok(m::Stmt::Ret(Some(lower_expr(value)?))),
        Stmt::Ret { value: None } => Ok(m::Stmt::Ret(None)),
        Stmt::VarDecl { name, value } => Ok(m::Stmt::VarDecl(name.to_string(), lower_expr(value)?)),
    }
}

fn lower_cases(cases: &[Case]) -> Res<Vec<m::Case>> {
    cases
        .iter()
        .map(|c| Ok((lower_expr(&c.expr)?, lower_stmts(&c.stmts)?)))
        .collect()
}

fn lower_exprs(exprs: &[Expr]) -> Res<Vec<m::Expr>> {
    exprs.iter().map(lower_expr).collect()
}

fn lower_expr(expr: &Expr) -> Res<m::Expr> {
    match expr {
        Expr::Const { value, r#type } => match (value, r#type) {
            (Value::Bool(b), Type::Bool) => Ok(m::Expr::ConstBool(*b)),
            (Value::Number(n), _) => Ok(lower_number(n, r#type)?),
            (Value::String(s), Type::Str) => Ok(m::Expr::ConstStr(s.to_string())),
            _ => Err(Box::from("Unsupported value and type")),
        },
        Expr::FuncCall { name, r#type, args } => Ok(m::Expr::FuncCall(
            name.to_string(),
            lower_type(r#type),
            lower_exprs(args)?,
        )),
        Expr::VarRef { name, r#type } => Ok(m::Expr::VarRef(name.to_string(), lower_type(r#type))),
    }
}

fn lower_number(num: &serde_json::value::Number, r#type: &Type) -> Res<m::Expr> {
    fn as_i32(num: &serde_json::value::Number) -> Res<i32> {
        num.as_i64()
            .map(i32::try_from)
            .and_then(|r| r.ok())
            .ok_or_else(|| Box::from("Number is not an Int32"))
    }

    fn as_i64(num: &serde_json::value::Number) -> Res<i64> {
        num.as_i64()
            .ok_or_else(|| Box::from("Number is not an Int64"))
    }

    fn as_f64(num: &serde_json::value::Number) -> Res<f64> {
        num.as_f64()
            .ok_or_else(|| Box::from("Number is not a Double"))
    }

    match (num, r#type) {
        (n, Type::Double) => Ok(m::Expr::ConstDouble(as_f64(n)?)),
        (n, Type::Int32) => Ok(m::Expr::ConstInt32(as_i32(n)?)),
        (n, Type::Int64) => Ok(m::Expr::ConstInt64(as_i64(n)?)),
        _ => Err(Box::from("Invalid number value and type")),
    }
}
