use midlang as m;

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

pub fn lower(midlang: &m::MidLang) -> LowerLang {
    match midlang {
        m::MidLang::Module(name, decls) => {
            LowerLang::CompUnit(name.to_string(), lower_decls(decls))
        }
    }
}

fn lower_decls(decls: &Vec<m::Decl>) -> Vec<Decl> {
    decls
        .iter()
        .filter_map(|d| match d {
            m::Decl::FuncDecl(name, visibility, r#type, args, stmts) => Some(Decl::FuncDecl(
                name.to_string(),
                lower_visibility(visibility),
                lower_type(r#type),
                lower_args(args),
                lower_stmts(stmts),
            )),
            m::Decl::FwdDecl(_, _, _, _) => None,
        })
        .collect()
}

fn lower_args(args: &m::FuncArgs) -> FuncArgs {
    fn lower(args: &Vec<m::FuncArg>) -> Vec<FuncArg> {
        args.iter().map(|a| lower_arg(a)).collect()
    }

    match args {
        m::FuncArgs::Fixed(args) => FuncArgs::Fixed(lower(args)),
        m::FuncArgs::Variadic(first, rest) => FuncArgs::Variadic(lower_arg(first), lower(rest)),
    }
}

fn lower_arg(arg: &m::FuncArg) -> FuncArg {
    match arg {
        m::FuncArg::Named(name, r#type) => FuncArg::Named(name.to_string(), lower_type(r#type)),
    }
}

fn lower_stmts(_stmts: &Vec<m::Stmt>) -> Vec<Stmt> {
    todo!()
}

fn lower_visibility(visibility: &m::Visibility) -> Option<Linkage> {
    match visibility {
        m::Visibility::Public => Some(Linkage::Export),
        m::Visibility::Private => None,
    }
}

fn lower_type(r#type: &m::Type) -> Type {
    match r#type {
        m::Type::Int32 => Type::W,
        m::Type::Str => Type::L,
    }
}
