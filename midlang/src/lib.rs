pub enum Type {
    Int32,
    Str,
}

pub enum Visibility {
    Public,
    Private,
}

pub enum MidLang {
    Module(String, Vec<Decl>),
}

pub enum FuncArg {
    Named(String, Type),
}

pub enum FuncArgs {
    Fixed(Vec<FuncArg>),
    Variadic(FuncArg, Vec<FuncArg>),
}

pub enum Decl {
    FwdDecl(String, Visibility, Type, FuncArgs),
    FuncDecl(String, Visibility, Type, FuncArgs, Vec<Stmt>),
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
