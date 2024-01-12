use midlang::middle_lang as m;

use crate::lower_lang::*;
use crate::lowering_context::LoweringCtx;

pub fn lower(modules: &[m::Module]) -> Vec<CompUnit> {
    modules
        .iter()
        .map(|m| {
            let lowered_decls = {
                let mut ctx = LoweringCtx::new(&m.name);
                let mut lowered_decls = lower_decls(&m.decls, &mut ctx);
                let mut vec = Vec::<Decl>::with_capacity(ctx.decls_len() + lowered_decls.len());

                vec.append(&mut ctx.decls());
                vec.append(&mut lowered_decls);

                vec
            };

            CompUnit {
                name: m.name.to_string(),
                decls: lowered_decls,
            }
        })
        .collect()
}

fn lower_decls(decls: &[m::Decl], ctx: &mut LoweringCtx) -> Vec<Decl> {
    decls
        .iter()
        .filter_map(|d| match d {
            m::Decl::FuncDecl(name, visibility, r#type, args, variadic, m_stmts) => {
                let mut stmts = Vec::<Stmt>::with_capacity(m_stmts.len() * 2);
                stmts.push(Stmt::Lbl("start".to_string()));
                lower_stmts(m_stmts, &mut stmts, ctx);

                Some(Decl::FuncDecl(
                    name.to_string(),
                    lower_visibility(visibility),
                    lower_type(r#type),
                    lower_args(args),
                    *variadic,
                    stmts,
                ))
            }
            m::Decl::FwdDecl(_, _, _, _, _) => None,
        })
        .collect()
}

fn lower_args(args: &[m::FuncArg]) -> Vec<FuncArg> {
    args.iter()
        .map(|a| (a.0.to_string(), lower_type(&a.1)))
        .collect()
}

fn lower_stmts(m_stmts: &[m::Stmt], stmts: &mut Vec<Stmt>, ctx: &mut LoweringCtx) {
    for stmt in m_stmts {
        match stmt {
            m::Stmt::Ret(expr) => {
                let value = lower_expr_to_value(expr, stmts, ctx);
                stmts.push(Stmt::Ret(value));
            }
            m::Stmt::VarDecl(name, expr) => {
                let expr = lower_expr(expr, stmts, ctx);
                stmts.push(Stmt::VarDecl(name.to_string(), Scope::Func, expr));
            }
        }
    }
}

fn lower_exprs_to_values(
    exprs: &[m::Expr],
    stmts: &mut Vec<Stmt>,
    ctx: &mut LoweringCtx,
) -> Vec<Value> {
    exprs
        .iter()
        .map(|e| lower_expr_to_value(e, stmts, ctx))
        .collect()
}

fn lower_expr_to_value(expr: &m::Expr, stmts: &mut Vec<Stmt>, ctx: &mut LoweringCtx) -> Value {
    match expr {
        m::Expr::ConstInt32(i) => Value::ConstW(*i),
        m::Expr::ConstStr(s) => {
            let name = ctx.name_for_str(s);
            Value::VarRef(name, Type::L, Scope::Global)
        }
        m::Expr::FuncCall(name, r#type, args) => {
            let expr = lower_func_call(name, r#type, args, stmts, ctx);
            let r#type = expr.r#type();
            let name = ctx.uniq_name("arg_");

            stmts.push(Stmt::VarDecl(name.to_string(), Scope::Func, expr));

            Value::VarRef(name, r#type, Scope::Func)
        }
    }
}

fn lower_expr(expr: &m::Expr, stmts: &mut Vec<Stmt>, ctx: &mut LoweringCtx) -> Expr {
    match expr {
        m::Expr::ConstInt32(_) | m::Expr::ConstStr(_) => {
            let value = lower_expr_to_value(expr, stmts, ctx);
            Expr::Value(value)
        }
        m::Expr::FuncCall(name, r#type, args) => lower_func_call(name, r#type, args, stmts, ctx),
    }
}

fn lower_func_call(
    name: &str,
    r#type: &m::Type,
    args: &[m::Expr],
    stmts: &mut Vec<Stmt>,
    ctx: &mut LoweringCtx,
) -> Expr {
    let values = lower_exprs_to_values(args, stmts, ctx);
    Expr::FuncCall(name.to_string(), lower_type(r#type), values)
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
