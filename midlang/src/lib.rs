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

pub enum Expr {
    ConstBool(bool),
    ConstInt32(i32),
    ConstInt64(i64),
    ConstStr(String),
    FuncCall(String, Type, Vec<Expr>),
    VarRef(String, Type),
}

#[derive(PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(PartialEq)]
pub enum Type {
    Bool,
    Int32,
    Int64,
    Ptr,
    Str,
}

impl Expr {
    pub fn r#type(&self) -> &Type {
        match self {
            Self::ConstBool(_) => &Type::Bool,
            Self::ConstInt32(_) => &Type::Int32,
            Self::ConstInt64(_) => &Type::Int64,
            Self::ConstStr(_) => &Type::Str,
            Self::FuncCall(_, r#type, _) => r#type,
            Self::VarRef(_, r#type) => r#type,
        }
    }
}
