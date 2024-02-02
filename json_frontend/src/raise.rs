use std::error::Error;

use serde_json::Value;

use midlang as m;

use crate::json_lang::*;

type Res<T> = Result<T, Box<dyn Error>>;

pub fn raise(modules: &[m::Module]) -> Res<JSONLang> {
    let raised: Vec<Module> = modules.iter().map(raise_module).collect();

    Ok(JSONLang::Modules(raised))
}

fn raise_module(module: &m::Module) -> Module {
    Module {
        name: module.name.to_string(),
        decls: raise_decls(&module.decls),
    }
}

fn raise_decls(decls: &[m::Decl]) -> Vec<Decl> {
    decls.iter().map(raise_decl).collect()
}

fn raise_decl(decl: &m::Decl) -> Decl {
    match decl {
        m::Decl::FwdDecl(name, visibility, r#type, args, variadic) => Decl::FwdDecl {
            name: name.to_string(),
            visibility: raise_visibility(visibility),
            r#type: raise_opt_type(r#type),
            args: raise_args(args),
            variadic: raise_opt_bool(variadic),
        },
        m::Decl::FuncDecl(name, visibility, r#type, args, variadic, stmts) => Decl::FuncDecl {
            name: name.to_string(),
            visibility: raise_visibility(visibility),
            r#type: raise_opt_type(r#type),
            args: raise_args(args),
            variadic: raise_opt_bool(variadic),
            stmts: raise_stmts(stmts),
        },
    }
}

fn raise_stmts(stmts: &[m::Stmt]) -> Vec<Stmt> {
    stmts.iter().map(raise_stmt).collect()
}

fn raise_stmt(stmt: &m::Stmt) -> Stmt {
    match stmt {
        m::Stmt::Cond(cases) => Stmt::Cond {
            cases: raise_cases(cases),
        },
        m::Stmt::FuncCall(name, args) => Stmt::FuncCall {
            name: name.to_string(),
            args: raise_exprs(args),
        },
        m::Stmt::Ret(Some(value)) => Stmt::Ret {
            value: Some(raise_expr(value)),
        },
        m::Stmt::Ret(None) => Stmt::Ret { value: None },
        m::Stmt::VarDecl(name, value) => Stmt::VarDecl {
            name: name.to_string(),
            value: raise_expr(value),
        },
    }
}

fn raise_cases(cases: &[m::Case]) -> Vec<Case> {
    cases
        .iter()
        .map(|(expr, stmts)| Case {
            expr: raise_expr(expr),
            stmts: raise_stmts(stmts),
        })
        .collect()
}

fn raise_exprs(exprs: &[m::Expr]) -> Vec<Expr> {
    exprs.iter().map(raise_expr).collect()
}

fn raise_expr(expr: &m::Expr) -> Expr {
    match expr {
        m::Expr::Cmp(m::Op::Eq, lhs, rhs) => Expr::Eq {
            lhs: Box::new(raise_expr(lhs)),
            rhs: Box::new(raise_expr(rhs)),
        },
        m::Expr::Cmp(m::Op::Ne, lhs, rhs) => Expr::Ne {
            lhs: Box::new(raise_expr(lhs)),
            rhs: Box::new(raise_expr(rhs)),
        },
        m::Expr::ConstBool(b) => Expr::Const {
            value: Value::from(*b),
            r#type: Type::Bool,
        },
        m::Expr::ConstDouble(d) => Expr::Const {
            value: Value::from(*d),
            r#type: Type::Double,
        },
        m::Expr::ConstInt32(i) => Expr::Const {
            value: Value::from(*i),
            r#type: Type::Int32,
        },
        m::Expr::ConstInt64(i) => Expr::Const {
            value: Value::from(*i),
            r#type: Type::Int64,
        },
        m::Expr::ConstStr(s) => Expr::Const {
            value: Value::from(s.to_string()),
            r#type: Type::Str,
        },
        m::Expr::FuncCall(name, r#type, args) => Expr::FuncCall {
            name: name.to_string(),
            r#type: raise_type(r#type),
            args: raise_exprs(args),
        },
        m::Expr::VarRef(name, r#type, byref) => Expr::VarRef {
            name: name.to_string(),
            r#type: raise_type(r#type),
            byref: raise_opt_bool(byref),
        },
    }
}

fn raise_args(args: &[m::FuncArg]) -> Vec<FuncArg> {
    args.iter()
        .map(|(name, r#type)| FuncArg {
            name: name.to_string(),
            r#type: raise_type(r#type),
        })
        .collect()
}

fn raise_opt_bool(value: &bool) -> Option<bool> {
    if *value {
        Some(true)
    } else {
        None
    }
}

fn raise_visibility(visibility: &m::Visibility) -> Visibility {
    match visibility {
        m::Visibility::Public => Visibility::Public,
        m::Visibility::Private => Visibility::Private,
    }
}

fn raise_type(r#type: &m::Type) -> Type {
    match r#type {
        m::Type::Bool => Type::Bool,
        m::Type::Double => Type::Double,
        m::Type::Int32 => Type::Int32,
        m::Type::Int64 => Type::Int64,
        m::Type::Ptr(Some(r#type)) => Type::Ptr {
            to: Box::new(raise_type(r#type)),
        },
        m::Type::Ptr(None) => Type::VoidPtr,
        m::Type::Str => Type::Str,
    }
}

fn raise_opt_type(r#type: &Option<m::Type>) -> Option<Type> {
    r#type.as_ref().map(raise_type)
}
