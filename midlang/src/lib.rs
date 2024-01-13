pub struct Module {
    pub name: String,
    pub decls: Vec<Decl>,
}

pub type FuncArg = (String, Type);
pub type Variadic = bool;

pub enum Decl {
    FwdDecl(String, Visibility, Type, Vec<FuncArg>, Variadic),
    FuncDecl(String, Visibility, Type, Vec<FuncArg>, Variadic, Vec<Stmt>),
}

pub enum Stmt {
    Ret(Expr),
    VarDecl(String, Expr),
}

pub enum Expr {
    ConstBool(bool),
    ConstInt32(i32),
    ConstInt64(i64),
    ConstStr(String),
    FuncCall(String, Type, Vec<Expr>),
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
        }
    }
}
