use midlang as m;

pub enum LowerLang {
    CompUnit(String, Vec<Decl>),
}

type DataField = (Type, String);

pub enum Decl {
    Data(String, Vec<DataField>),
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
    Ret(Value),
    VarDecl(String, Scope, Expr),
}

pub enum Expr {
    Value(Value),
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

fn lower_stmts(stmts: &Vec<m::Stmt>) -> Vec<Stmt> {
    stmts
        .iter()
        .flat_map(|s| match s {
            m::Stmt::Ret(expr) => {
                let (mut stmts, value) = lower_expr_to_value(expr);
                stmts.push(Stmt::Ret(value));
                stmts
            }
            m::Stmt::VarDecl(name, expr) => {
                let (mut stmts, expr) = lower_expr(expr);
                stmts.push(Stmt::VarDecl(name.to_string(), Scope::Func, expr));
                stmts
            }
        })
        .collect()
}

fn lower_exprs_to_values(_exprs: &Vec<m::Expr>) -> (Vec<Stmt>, Vec<Value>) {
    todo!()
}

fn lower_expr_to_value(_expr: &m::Expr) -> (Vec<Stmt>, Value) {
    todo!()
}

fn lower_expr(expr: &m::Expr) -> (Vec<Stmt>, Expr) {
    match expr {
        m::Expr::ConstInt32(i) => (vec![], Expr::Value(Value::ConstW(*i))),
        m::Expr::ConstStr(_) => todo!(),
        m::Expr::FuncCall(name, r#type, args) => {
            let (stmts, values) = lower_exprs_to_values(args);
            (
                stmts,
                Expr::FuncCall(name.to_string(), lower_type(r#type), values),
            )
        }
    }
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

use std::collections::BTreeMap;

pub struct StringPool {
    prefix: String,
    pool: BTreeMap<String, String>,
}

impl StringPool {
    pub fn new(prefix: &str) -> StringPool {
        StringPool {
            prefix: prefix.to_string(),
            pool: Default::default(),
        }
    }

    pub fn name_for_str(mut self, str: &str) -> String {
        let len = &self.pool.len();
        self.pool
            .entry(str.to_string())
            .or_insert_with(|| format!("{}_{}", self.prefix, len))
            .to_string()
    }

    pub fn as_data(self) -> Vec<Decl> {
        fn fields(value: &str) -> Vec<DataField> {
            vec![(Type::B, value.to_string()), (Type::B, "0".to_string())]
        }

        self.pool
            .iter()
            .map(|(k, v)| Decl::Data(k.to_string(), fields(v)))
            .collect()
    }
}
