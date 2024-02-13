use std::fmt;
use std::fmt::{Display, Formatter};

pub struct Module {
    pub name: String,
    pub decls: Vec<Decl>,
}

pub type Case = (Expr, Vec<Stmt>);
pub type FuncArg = (String, Type);
pub type Variadic = bool;

pub enum Decl {
    FwdDecl(String, Visibility, Option<Type>, Vec<FuncArg>, Variadic),
    FuncDecl(
        String,
        Visibility,
        Option<Type>,
        Vec<FuncArg>,
        Variadic,
        Vec<Stmt>,
    ),
}

pub enum Stmt {
    Cond(Vec<Case>),
    FuncCall(String, Vec<Expr>),
    Ret(Option<Expr>),
    VarDecl(String, Expr),
}

#[derive(Debug)]
pub enum Expr {
    Cmp(Op, Box<Expr>, Box<Expr>),
    ConstBool(bool),
    ConstDouble(f64),
    ConstInt32(i32),
    ConstInt64(i64),
    ConstStr(String),
    FuncCall(String, Type, Vec<Expr>),
    Not(Box<Expr>),
    VarRef(String, Type, bool),
}

#[derive(Debug)]
pub enum Op {
    Eq,
    Ne,
}

#[derive(PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Bool,
    Double,
    Int32,
    Int64,
    Ptr(Option<Box<Type>>),
    Str,
}

impl Expr {
    pub fn r#type(&self) -> &Type {
        match self {
            Self::Cmp(_, _, _) => &Type::Bool,
            Self::ConstBool(_) => &Type::Bool,
            Self::ConstDouble(_) => &Type::Double,
            Self::ConstInt32(_) => &Type::Int32,
            Self::ConstInt64(_) => &Type::Int64,
            Self::ConstStr(_) => &Type::Str,
            Self::FuncCall(_, r#type, _) => r#type,
            Self::Not(_) => &Type::Bool,
            Self::VarRef(_, r#type, _) => r#type,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "eq"),
            Self::Ne => write!(f, "ne"),
        }
    }
}
