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
    Variadic(Vec<FuncArg>),
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

pub type ParseResult = Result<JSONLang, Box<dyn Error>>;

fn parse_file(path: &PathBuf) -> ParseResult {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result = serde_json::from_reader(reader)?;

    Ok(result)
}

pub fn parse_file_named(name: &str) -> ParseResult {
    let path = PathBuf::from(name);
    parse_file(&path)
}

#[derive(Default)]
pub struct LoweringCtx<'a> {
    decls: Vec<m::Decl<'a>>,
}

pub fn lower<'a>(
    json_lang: &'a JSONLang,
    ctx: &'a mut LoweringCtx<'a>,
) -> Result<m::MidLang<'a>, Box<dyn Error>> {
    match json_lang {
        JSONLang::Module { name, decls } => Ok(m::MidLang::Module(name, lower_decls(&decls, ctx)?)),
    }
}

fn lower_decls<'a>(
    decls: &'a [Decl],
    ctx: &'a mut LoweringCtx<'a>,
) -> Result<&'a [m::Decl<'a>], Box<dyn Error>> {
    let start_idx = ctx.decls.len();
    let mut m_decls = Vec::<m::Decl>::new();

    for decl in decls {
        let m_decl = lower_decl(decl)?;
        m_decls.push(m_decl);
    }

    ctx.decls.append(&mut m_decls);

    let end_idx = ctx.decls.len();

    Ok(&ctx.decls[start_idx..end_idx])
}

fn lower_decl<'a>(decl: &Decl) -> Result<m::Decl<'a>, Box<dyn Error>> {
    match decl {
        Decl::FwdDecl {
            name,
            visibility,
            r#type,
            args,
        } => todo!(),
        Decl::FuncDecl {
            name,
            visibility,
            r#type,
            args,
            stmts,
        } => todo!(),
    }
}

fn lower_func_args<'a>(
    _args: &'a FuncArgs,
    _ctx: &'a mut LoweringCtx<'a>,
) -> Result<&'a m::FuncArgs<'a>, Box<dyn Error>> {
    todo!();
}

fn lower_stmts<'a>(
    _stmts: &'a [Stmt],
    _ctx: &'a mut LoweringCtx<'a>,
) -> Result<&'a [m::Stmt<'a>], Box<dyn Error>> {
    todo!();
}

fn lower_exprs<'a>(
    _exprs: &'a [Expr],
    _ctx: &'a mut LoweringCtx<'a>,
) -> Result<&'a [m::Expr<'a>], Box<dyn Error>> {
    todo!();
}

fn lower_type(r#type: Type) -> m::Type {
    match r#type {
        Type::Int32 => m::Type::Int32,
        Type::Str => m::Type::Str,
    }
}

fn lower_visibility(visibility: Visibility) -> m::Visibility {
    match visibility {
        Visibility::Public => m::Visibility::Public,
        Visibility::Private => m::Visibility::Private,
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
