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

#[derive(Clone, Copy)]
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

pub trait Typed {
    fn r#type(&self) -> Type;
}

impl Typed for Expr {
    fn r#type(&self) -> Type {
        match self {
            Expr::Value(value) => value.r#type(),
            Expr::FuncCall(_, r#type, _) => *r#type,
        }
    }
}

impl Typed for Value {
    fn r#type(&self) -> Type {
        match self {
            Value::ConstW(_) => Type::W,
            Value::VarRef(_, r#type, _) => *r#type,
        }
    }
}

pub fn lower(midlang: &m::MidLang) -> LowerLang {
    match midlang {
        m::MidLang::Module(name, decls) => {
            let lowered_decls = {
                let mut str_pool = StringPool::new(name);
                let mut lowered_decls = lower_decls(decls, &mut str_pool);
                let mut vec = Vec::<Decl>::with_capacity(str_pool.pool.len() + lowered_decls.len());

                vec.append(&mut str_pool.decls());
                vec.append(&mut lowered_decls);

                vec
            };

            LowerLang::CompUnit(name.to_string(), lowered_decls)
        }
    }
}

fn lower_decls(decls: &Vec<m::Decl>, mut str_pool: &mut StringPool) -> Vec<Decl> {
    decls
        .iter()
        .filter_map(|d| match d {
            m::Decl::FuncDecl(name, visibility, r#type, args, stmts) => Some(Decl::FuncDecl(
                name.to_string(),
                lower_visibility(visibility),
                lower_type(r#type),
                lower_args(args),
                lower_stmts(stmts, &mut str_pool),
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

fn lower_stmts(stmts: &Vec<m::Stmt>, str_pool: &mut StringPool) -> Vec<Stmt> {
    stmts
        .iter()
        .flat_map(|s| match s {
            m::Stmt::Ret(expr) => {
                let (mut stmts, value) = lower_expr_to_value(expr, str_pool);
                stmts.push(Stmt::Ret(value));
                stmts
            }
            m::Stmt::VarDecl(name, expr) => {
                let (mut stmts, expr) = lower_expr(expr, str_pool);
                stmts.push(Stmt::VarDecl(name.to_string(), Scope::Func, expr));
                stmts
            }
        })
        .collect()
}

fn lower_exprs_to_values(
    exprs: &Vec<m::Expr>,
    str_pool: &mut StringPool,
) -> (Vec<Stmt>, Vec<Value>) {
    let (stmts, values): (Vec<Vec<_>>, Vec<_>) = exprs
        .iter()
        .map(|e| lower_expr_to_value(e, str_pool))
        .unzip();
    let stmts = stmts.into_iter().flatten().collect();

    (stmts, values)
}

fn lower_expr_to_value(expr: &m::Expr, str_pool: &mut StringPool) -> (Vec<Stmt>, Value) {
    match expr {
        m::Expr::ConstInt32(i) => (vec![], Value::ConstW(*i)),
        m::Expr::ConstStr(s) => {
            let name = str_pool.name_for_str(s);
            (vec![], Value::VarRef(name, Type::L, Scope::Global))
        }
        m::Expr::FuncCall(name, r#type, args) => {
            let (mut stmts, expr) = lower_func_call(name, r#type, args, str_pool);
            let r#type = expr.r#type();
            let name = format!("a{}", stmts.len());

            // todo: this isn't right, need to port the "new_name" helper
            stmts.push(Stmt::VarDecl(name.to_string(), Scope::Func, expr));

            (stmts, Value::VarRef(name, r#type, Scope::Func))
        }
    }
}

fn lower_expr(expr: &m::Expr, str_pool: &mut StringPool) -> (Vec<Stmt>, Expr) {
    match expr {
        m::Expr::ConstInt32(_) | m::Expr::ConstStr(_) => {
            let (stmts, value) = lower_expr_to_value(expr, str_pool);
            (stmts, Expr::Value(value))
        }
        m::Expr::FuncCall(name, r#type, args) => lower_func_call(name, r#type, args, str_pool),
    }
}

fn lower_func_call(
    name: &str,
    r#type: &m::Type,
    args: &Vec<m::Expr>,
    str_pool: &mut StringPool,
) -> (Vec<Stmt>, Expr) {
    let (stmts, values) = lower_exprs_to_values(args, str_pool);
    (
        stmts,
        Expr::FuncCall(name.to_string(), lower_type(r#type), values),
    )
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

    pub fn name_for_str(&mut self, str: &str) -> String {
        let len = &self.pool.len();
        self.pool
            .entry(str.to_string())
            .or_insert_with(|| format!("{}_{}", self.prefix, len))
            .to_string()
    }

    pub fn decls(&self) -> Vec<Decl> {
        fn fields(value: &str) -> Vec<DataField> {
            vec![(Type::B, value.to_string()), (Type::B, "0".to_string())]
        }

        self.pool
            .iter()
            .map(|(k, v)| Decl::Data(k.to_string(), fields(v)))
            .collect()
    }
}
