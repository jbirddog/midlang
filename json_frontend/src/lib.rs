use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json;
use serde_json::Value;

use midlang as m;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JSONLang {
    Module { name: String, decls: Vec<Decl> },
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Decl {
    FwdDecl {
        name: String,
        visibility: Visibility,
        r#type: Type,
        args: FuncArgs,
    },
    FuncDecl {
        name: String,
        visibility: Visibility,
        r#type: Type,
        args: FuncArgs,
        stmts: Vec<Stmt>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Int32,
    Str,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FuncArg {
    Named { name: String, r#type: Type },
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FuncArgs {
    Fixed(Vec<FuncArg>),
    Variadic(FuncArg, Vec<FuncArg>),
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stmt {
    Ret { value: Expr },
    VarDecl { name: String, value: Expr },
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Expr {
    Const {
        value: Value,
        r#type: Type,
    },
    FuncCall {
        name: String,
        r#type: Type,
        args: Vec<Expr>,
    },
}

type Res<T> = Result<T, Box<dyn Error>>;

pub fn parse_file_named(name: &str) -> Res<JSONLang> {
    let path = PathBuf::from(name);
    parse_file(&path)
}

fn parse_file(path: &PathBuf) -> Res<JSONLang> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader)?;

    Ok(result)
}

pub fn lower(json_lang: &JSONLang) -> Res<m::MidLang> {
    let module = match json_lang {
        JSONLang::Module { name, decls } => {
            m::MidLang::Module(name.to_string(), lower_decls(decls)?)
        }
    };

    Ok(module)
}

fn lower_decls(decls: &Vec<Decl>) -> Res<Vec<m::Decl>> {
    decls.iter().map(|d| lower_decl(d)).collect()
}

fn lower_decl(decl: &Decl) -> Res<m::Decl> {
    match decl {
        Decl::FwdDecl {
            name,
            visibility,
            r#type,
            args,
        } => Ok(m::Decl::FwdDecl(
            name.to_string(),
            lower_visibility(visibility),
            lower_type(r#type),
            lower_args(args),
        )),
        Decl::FuncDecl {
            name,
            visibility,
            r#type,
            args,
            stmts,
        } => Ok(m::Decl::FuncDecl(
            name.to_string(),
            lower_visibility(visibility),
            lower_type(r#type),
            lower_args(args),
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

fn lower_args(args: &FuncArgs) -> m::FuncArgs {
    fn lower(args: &Vec<FuncArg>) -> Vec<m::FuncArg> {
        args.iter().map(|a| lower_arg(a)).collect()
    }

    match args {
        FuncArgs::Fixed(args) => m::FuncArgs::Fixed(lower(args)),
        FuncArgs::Variadic(first, rest) => m::FuncArgs::Variadic(lower_arg(first), lower(rest)),
    }
}

fn lower_arg(arg: &FuncArg) -> m::FuncArg {
    match arg {
        FuncArg::Named { name, r#type } => m::FuncArg::Named(name.to_string(), lower_type(r#type)),
    }
}

fn lower_stmts(stmts: &Vec<Stmt>) -> Res<Vec<m::Stmt>> {
    stmts.iter().map(|s| lower_stmt(s)).collect()
}

fn lower_stmt(stmt: &Stmt) -> Res<m::Stmt> {
    match stmt {
        Stmt::Ret { value } => Ok(m::Stmt::Ret(lower_expr(value)?)),
        Stmt::VarDecl { name, value } => Ok(m::Stmt::VarDecl(name.to_string(), lower_expr(value)?)),
    }
}

fn lower_exprs(exprs: &Vec<Expr>) -> Res<Vec<m::Expr>> {
    exprs.iter().map(|e| lower_expr(e)).collect()
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
            .map(|i| i32::try_from(i))
            .map(|r| r.ok())
            .flatten()
            .ok_or_else(|| Box::from("Number is not an Int32"))
    }

    match (num, r#type) {
        (n, Type::Int32) => Ok(m::Expr::ConstInt32(as_i32(n)?)),
        _ => Err(Box::from("Invalid number value and type")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    fn hello_world() -> TestResult {
        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("json")
            .join("hello_world.json");

        parse_file(&path)?;

        Ok(())
    }
}
