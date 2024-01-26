use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Module {
    pub name: String,
    pub decls: Vec<Decl>,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JSONLang {
    Modules(Vec<Module>),
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Decl {
    FwdDecl {
        name: String,
        visibility: Visibility,
        r#type: Option<Type>,
        args: Vec<FuncArg>,
        variadic: Option<bool>,
    },
    FuncDecl {
        name: String,
        visibility: Visibility,
        r#type: Option<Type>,
        args: Vec<FuncArg>,
        variadic: Option<bool>,
        stmts: Vec<Stmt>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct FuncArg {
    pub name: String,
    pub r#type: Type,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Stmt {
    Cond { cases: Vec<Case> },
    FuncCall { name: String, args: Vec<Expr> },
    Ret { value: Option<Expr> },
    VarDecl { name: String, value: Expr },
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Case {
    pub expr: Expr,
    pub stmts: Vec<Stmt>,
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
    VarRef {
        name: String,
        r#type: Type,
        byref: Option<bool>,
    },
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Bool,
    Double,
    Int32,
    Int64,
    Ptr { to: Box<Type> },
    Str,
    VoidPtr,
}
