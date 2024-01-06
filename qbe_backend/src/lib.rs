pub enum LowerLang {
    CompUnit(String, Vec<Decl>),
}

pub enum Decl {
    Data(String, Vec<(Type, String)>),
    FuncDecl(String, Option<Linkage>, Type, FuncArgs, Vec<Stmt>),
}

pub enum FuncArgs {
    Fixed(Vec<FuncArg>),
    Variadic(FuncArg, Vec<FuncArg>),
}

pub enum FuncArg {
    Named(String, Type),
}

pub enum Stmt {
    FuncCall(String, Vec<Value>),
    Jmp(String),
    Jnz(Expr, String, String),
    Lbl(String),
    Ret(Expr),
    VarDecl(String, Type, Scope, Expr),
}

pub enum Expr {
    Value,
    FuncCall(String, Type, Vec<Value>),
}

pub enum Value {
    ConstW(i32),
    VarRef(String, Type, Scope),
}

pub enum Linkage {
    Export,
}

pub enum Type {
    B,
    D,
    H,
    L,
    S,
    W,
}

pub enum Scope {
    Func,
    Global,
}
