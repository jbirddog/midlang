use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json;

use midlang;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum MidLang {
    Module { name: String, decls: Vec<Decl> },
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Decl {
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
enum Type {
    Int32,
    Str,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum FuncArg {
    Named { name: String, r#type: Type },
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum FuncArgs {
    Fixed(Vec<FuncArg>),
    Variadic(Vec<FuncArg>),
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Stmt {
    Ret { value: Expr },
    VarDecl { name: String, value: Expr },
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Expr {
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

pub fn parse_file<T>(path: &PathBuf) -> Result<T, Box<dyn Error>>
where
    T: serde::de::DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let result: T = serde_json::from_reader(reader)?;

    Ok(result)
}

pub fn parse_string<'a, T>(str: &'a str) -> serde_json::Result<T>
where
    T: Deserialize<'a>,
{
    serde_json::from_str::<T>(str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn empty_module() -> serde_json::Result<()> {
        let json = "{\"module\": {\"name\": \"empty\", \"decls\": []}}";

        match parse_string::<MidLang>(json)? {
            MidLang::Module { name, decls } => {
                assert_eq!(name.as_str(), "empty");
                assert_eq!(decls.len(), 0);
            }
        }

        Ok(())
    }

    type TestResult<T> = Result<T, Box<dyn Error>>;

    #[test]
    fn hello_world() -> TestResult<()> {
        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("json")
            .join("hello_world.json");
        let result = parse_file::<MidLang>(&path)?;
        Ok(())
    }
}
