use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json;

use midlang::MidLang;

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
        visibility: String,
        r#type: Type,
        args: FuncArgs,
    },
    FuncDecl {
        name: String,
        visibility: String,
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

pub type LowerResult<'a> = Result<MidLang<'a>, Box<dyn Error>>;

pub fn lower(json_lang: &JSONLang) -> LowerResult {
    let _ = MidLang::Module("bob", &[]);
    match json_lang {
        JSONLang::Module { name, .. } => Ok(MidLang::Module(name, &[])),
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
