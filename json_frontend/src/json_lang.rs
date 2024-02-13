use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Module {
    pub name: String,
    pub decls: Vec<Decl>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum JSONLang {
    Modules(Vec<Module>),
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Decl {
    FwdDecl {
        name: String,
        visibility: Visibility,

        #[serde(skip_serializing_if = "Option::is_none")]
        r#type: Option<Type>,

        args: Vec<FuncArg>,

        #[serde(skip_serializing_if = "Option::is_none")]
        variadic: Option<bool>,
    },
    FuncDecl {
        name: String,
        visibility: Visibility,

        #[serde(skip_serializing_if = "Option::is_none")]
        r#type: Option<Type>,

        args: Vec<FuncArg>,

        #[serde(skip_serializing_if = "Option::is_none")]
        variadic: Option<bool>,

        stmts: Vec<Stmt>,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct FuncArg {
    pub name: String,
    pub r#type: Type,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Stmt {
    Cond { cases: Vec<Case> },
    FuncCall { name: String, args: Vec<Expr> },
    Ret { value: Option<Expr> },
    VarDecl { name: String, value: Expr },
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Case {
    pub expr: Expr,
    pub stmts: Vec<Stmt>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Expr {
    Eq {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Const {
        value: Value,
        r#type: Type,
    },
    FuncCall {
        name: String,
        r#type: Type,
        args: Vec<Expr>,
    },
    Ne {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Not {
        expr: Box<Expr>,
    },
    VarRef {
        name: String,
        r#type: Type,

        #[serde(skip_serializing_if = "Option::is_none")]
        byref: Option<bool>,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Deserialize, Serialize)]
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
