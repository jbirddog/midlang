pub enum Type {
    Int32,
    Str,
}

pub enum Visibility {
    Public,
    Private,
}

pub enum MidLang<'a> {
    Module(&'a str, &'a [Decl<'a>]),
}

pub enum FuncArg<'a> {
    Named(&'a str, Type),
}

pub enum FuncArgs<'a> {
    Fixed(&'a [FuncArg<'a>]),
    Variadic(FuncArg<'a>, &'a [FuncArg<'a>]),
}

pub enum Decl<'a> {
    FwdDecl(&'a str, Visibility, Type, FuncArgs<'a>),
    FuncDecl(&'a str, Visibility, Type, FuncArgs<'a>, &'a [Stmt<'a>]),
}

pub enum Stmt<'a> {
    Ret(Expr<'a>),
    VarDecl(&'a str, Expr<'a>),
}

pub enum Expr<'a> {
    ConstInt32(i32),
    ConstStr(&'a str),
    FuncCall(&'a str, Type, &'a [Expr<'a>]),
}
