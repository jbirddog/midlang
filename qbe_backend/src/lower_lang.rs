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
        Option<Type>,
        Vec<FuncArg>,
        Variadic,
        Vec<Stmt>,
    ),
}

pub enum Stmt {
    FuncCall(String, Vec<Value>),
    Jmp(String),
    Jnz(Value, String, String),
    Lbl(String),
    Ret(Option<Value>),
    Store(Type, Value, Value),
    VarDecl(String, Scope, Expr),
}

pub enum Expr {
    Alloc8(usize),
    Cmp(Op, Value, Value),
    Load(Type, Type, Value),
    Sub(Value, Value),
    Value(Value),
    FuncCall(String, Type, Vec<Value>),
}

pub enum Value {
    ConstD(f64),
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
    D,
    L,
    W,
}

pub enum Scope {
    Func,
    Global,
}

pub enum Op {
    Eq,
    Ne,
}

pub trait Typed {
    fn r#type(&self) -> Type;
}

impl Typed for Expr {
    fn r#type(&self) -> Type {
        match self {
            Expr::Alloc8(_) => Type::L,
            Expr::Cmp(_, _, _) => Type::W,
            Expr::Load(r#type, _, _) => *r#type,
            Expr::Sub(value, _) => value.r#type(),
            Expr::Value(value) => value.r#type(),
            Expr::FuncCall(_, r#type, _) => *r#type,
        }
    }
}

impl Typed for Value {
    fn r#type(&self) -> Type {
        match self {
            Value::ConstD(_) => Type::D,
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

impl Display for Op {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "eq"),
            Self::Ne => write!(f, "ne"),
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
            Self::D => write!(f, "d"),
            Self::L => write!(f, "l"),
            Self::W => write!(f, "w"),
        }
    }
}
