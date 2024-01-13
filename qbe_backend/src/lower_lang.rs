use std::fmt;
use std::fmt::{Display, Formatter};

pub struct CompUnit {
    pub name: String,
    pub decls: Vec<Decl>,
}

pub type DataField = (Type, String);
pub type FuncArg = (String, Type);
pub type Variadic = bool;

pub enum Decl {
    Data(String, Vec<DataField>),
    FuncDecl(
        String,
        Option<Linkage>,
        Type,
        Vec<FuncArg>,
        Variadic,
        Vec<Stmt>,
    ),
}

pub enum Stmt {
    Lbl(String),
    Ret(Value),
    VarDecl(String, Scope, Expr),
}

pub enum Expr {
    Value(Value),
    FuncCall(String, Type, Vec<Value>),
}

pub enum Value {
    ConstL(i64),
    ConstW(i32),
    VarRef(String, Type, Scope),
}

pub enum Linkage {
    Export,
}

#[derive(Clone, Copy)]
pub enum Type {
    B,
    L,
    W,
}

pub enum Scope {
    Func,
    Global,
}

pub trait Typed {
    fn r#type(&self) -> Type;
}

impl Typed for Expr {
    fn r#type(&self) -> Type {
        match self {
            Expr::Value(value) => value.r#type(),
            Expr::FuncCall(_, r#type, _) => *r#type,
        }
    }
}

impl Typed for Value {
    fn r#type(&self) -> Type {
        match self {
            Value::ConstL(_) => Type::L,
            Value::ConstW(_) => Type::W,
            Value::VarRef(_, r#type, _) => *r#type,
        }
    }
}

impl Display for Linkage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Export => write!(f, "export"),
        }
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Func => write!(f, "%"),
            Self::Global => write!(f, "$"),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::B => write!(f, "b"),
            Self::L => write!(f, "l"),
            Self::W => write!(f, "w"),
        }
    }
}
