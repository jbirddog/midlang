use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json;

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
        value: serde_json::Value,
        r#type: Type,
    },
    FuncCall {
        name: String,
        r#type: Type,
        args: Vec<Expr>,
    },
}

type Res<T> = Result<T, Box<dyn Error>>;

fn parse_file(path: &PathBuf) -> Res<JSONLang> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader)?;

    Ok(result)
}

pub fn parse_file_named(name: &str) -> Res<JSONLang> {
    let path = PathBuf::from(name);
    parse_file(&path)
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

fn lower_stmts(_stmts: &Vec<Stmt>) -> Res<Vec<m::Stmt>> {
    todo!()
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
