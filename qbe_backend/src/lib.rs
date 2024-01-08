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
                let mut ctx = LoweringCtx::new(name);
                let mut lowered_decls = lower_decls(decls, &mut ctx);
                let mut vec = Vec::<Decl>::with_capacity(ctx.pool.len() + lowered_decls.len());

                vec.append(&mut ctx.decls());
                vec.append(&mut lowered_decls);

                vec
            };

            LowerLang::CompUnit(name.to_string(), lowered_decls)
        }
    }
}

fn lower_decls(decls: &Vec<m::Decl>, mut ctx: &mut LoweringCtx) -> Vec<Decl> {
    decls
        .iter()
        .filter_map(|d| match d {
            m::Decl::FuncDecl(name, visibility, r#type, args, stmts) => Some(Decl::FuncDecl(
                name.to_string(),
                lower_visibility(visibility),
                lower_type(r#type),
                lower_args(args),
                lower_stmts(stmts, &mut ctx),
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

fn lower_stmts(stmts: &Vec<m::Stmt>, ctx: &mut LoweringCtx) -> Vec<Stmt> {
    stmts
        .iter()
        .flat_map(|s| match s {
            m::Stmt::Ret(expr) => {
                let (mut stmts, value) = lower_expr_to_value(expr, ctx);
                stmts.push(Stmt::Ret(value));
                stmts
            }
            m::Stmt::VarDecl(name, expr) => {
                let (mut stmts, expr) = lower_expr(expr, ctx);
                stmts.push(Stmt::VarDecl(name.to_string(), Scope::Func, expr));
                stmts
            }
        })
        .collect()
}

fn lower_exprs_to_values(exprs: &Vec<m::Expr>, ctx: &mut LoweringCtx) -> (Vec<Stmt>, Vec<Value>) {
    let (stmts, values): (Vec<Vec<_>>, Vec<_>) =
        exprs.iter().map(|e| lower_expr_to_value(e, ctx)).unzip();
    let stmts = stmts.into_iter().flatten().collect();

    (stmts, values)
}

fn lower_expr_to_value(expr: &m::Expr, ctx: &mut LoweringCtx) -> (Vec<Stmt>, Value) {
    match expr {
        m::Expr::ConstInt32(i) => (vec![], Value::ConstW(*i)),
        m::Expr::ConstStr(s) => {
            let name = ctx.name_for_str(s);
            (vec![], Value::VarRef(name, Type::L, Scope::Global))
        }
        m::Expr::FuncCall(name, r#type, args) => {
            let (mut stmts, expr) = lower_func_call(name, r#type, args, ctx);
            let r#type = expr.r#type();
            let name = ctx.uniq_name("arg_");

            stmts.push(Stmt::VarDecl(name.to_string(), Scope::Func, expr));

            (stmts, Value::VarRef(name, r#type, Scope::Func))
        }
    }
}

fn lower_expr(expr: &m::Expr, ctx: &mut LoweringCtx) -> (Vec<Stmt>, Expr) {
    match expr {
        m::Expr::ConstInt32(_) | m::Expr::ConstStr(_) => {
            let (stmts, value) = lower_expr_to_value(expr, ctx);
            (stmts, Expr::Value(value))
        }
        m::Expr::FuncCall(name, r#type, args) => lower_func_call(name, r#type, args, ctx),
    }
}

fn lower_func_call(
    name: &str,
    r#type: &m::Type,
    args: &Vec<m::Expr>,
    ctx: &mut LoweringCtx,
) -> (Vec<Stmt>, Expr) {
    let (stmts, values) = lower_exprs_to_values(args, ctx);
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

pub struct LoweringCtx {
    prefix: String,
    pool: BTreeMap<String, String>,
    uniq: u32,
}

impl LoweringCtx {
    pub fn new(prefix: &str) -> LoweringCtx {
        LoweringCtx {
            prefix: prefix.to_string(),
            pool: Default::default(),
            uniq: 0,
        }
    }

    pub fn uniq_name(&mut self, prefix: &str) -> String {
        let name = format!("{}{}", prefix, self.uniq);
        self.uniq += 1;
        name
    }

    pub fn name_for_str(&mut self, str: &str) -> String {
        let len = self.pool.len();
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
