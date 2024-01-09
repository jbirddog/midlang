mod lower_lang;
mod lowering_context;

use lower_lang::*;
use lowering_context::LoweringCtx;

use midlang::middle_lang as m;

pub fn lower(midlang: &m::MidLang) -> LowerLang {
    match midlang {
        m::MidLang::Module(name, decls) => {
            let lowered_decls = {
                let mut ctx = LoweringCtx::new(name);
                let mut lowered_decls = lower_decls(decls, &mut ctx);
                let mut vec = Vec::<Decl>::with_capacity(ctx.decls_len() + lowered_decls.len());

                vec.append(&mut ctx.decls());
                vec.append(&mut lowered_decls);

                vec
            };

            LowerLang::CompUnit(name.to_string(), lowered_decls)
        }
    }
}

fn lower_decls(decls: &[m::Decl], ctx: &mut LoweringCtx) -> Vec<Decl> {
    decls
        .iter()
        .filter_map(|d| match d {
            m::Decl::FuncDecl(name, visibility, r#type, args, stmts) => Some(Decl::FuncDecl(
                name.to_string(),
                lower_visibility(visibility),
                lower_type(r#type),
                lower_args(args),
                lower_stmts(stmts, ctx),
            )),
            m::Decl::FwdDecl(_, _, _, _) => None,
        })
        .collect()
}

fn lower_args(args: &m::FuncArgs) -> FuncArgs {
    fn lower(args: &[m::FuncArg]) -> Vec<FuncArg> {
        args.iter().map(lower_arg).collect()
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

fn lower_stmts(stmts: &[m::Stmt], ctx: &mut LoweringCtx) -> Vec<Stmt> {
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

fn lower_exprs_to_values(exprs: &[m::Expr], ctx: &mut LoweringCtx) -> (Vec<Stmt>, Vec<Value>) {
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
    args: &[m::Expr],
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
