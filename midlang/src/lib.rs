pub enum Type {
    Int32,
    Str,
}

pub enum Visibility {
    Public,
    Private,
}

type Name = String;
type Stmts = Box<[Stmt]>;

pub struct FuncArg {
    name: String,
    r#type: Type,
}

pub enum FuncArgs {
    None,
    Fixed(Box<[FuncArg]>),
    Variadic(FuncArg, Box<[FuncArg]>),
}

pub struct Module {
    name: String,
    decls: Box<[Decl]>,
}

pub enum Decl {
    Extern(Name, Type, FuncArgs),
    FwdDecl(Name, Visibility, Type, FuncArgs),
    FuncDecl(Name, Visibility, Type, FuncArgs, Stmts),
}

pub enum Stmt {
    Ret(Expr),
}

pub enum Expr {
    ConstInt32(i32),
}
