pub struct Module {
    pub name: String,
    pub decls: Vec<Decl>,
}

pub enum Decl {
    FwdDecl(String, Visibility, Type, FuncArgs),
    FuncDecl(String, Visibility, Type, FuncArgs, Vec<Stmt>),
}

#[derive(PartialEq)]
pub enum FuncArgs {
    Fixed(Vec<FuncArg>),
    Variadic(FuncArg, Vec<FuncArg>),
}

#[derive(PartialEq)]
pub enum FuncArg {
    Named(String, Type),
}

pub enum Stmt {
    Ret(Expr),
    VarDecl(String, Expr),
}

pub enum Expr {
    ConstInt32(i32),
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
    Int32,
    Str,
}

impl Expr {
    pub fn r#type(&self) -> &Type {
        match self {
            Self::ConstInt32(_) => &Type::Int32,
            Self::ConstStr(_) => &Type::Str,
            Self::FuncCall(_, r#type, _) => r#type,
        }
    }
}

impl FuncArg {
    pub fn name(&self) -> &str {
        match self {
            Self::Named(name, _) => name,
        }
    }

    pub fn r#type(&self) -> &Type {
        match self {
            Self::Named(_, r#type) => r#type,
        }
    }
}

impl FuncArgs {
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Fixed(v) => v.is_empty(),
            Self::Variadic(_, _) => false,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Fixed(v) => v.len(),
            Self::Variadic(_, v) => v.len() + 1,
        }
    }
}
