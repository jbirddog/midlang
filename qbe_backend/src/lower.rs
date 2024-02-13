use midlang as m;

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
                stmts.push(lbl("start"));
                lower_stmts(m_stmts, &mut stmts, ctx);

                Some(Decl::FuncDecl(
                    name.to_string(),
                    lower_visibility(visibility),
                    lower_opt_type(r#type),
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
        ctx.push_tmp_refs();

        match stmt {
            m::Stmt::Cond(cases) => {
                let lbl_prefix = ctx.uniq_name("cond");
                let end_lbl = format!("{}_end", lbl_prefix);

                for (i, (expr, case_stmts)) in cases.iter().enumerate() {
                    let value = lower_expr_to_value(expr, stmts, ctx);
                    let true_lbl = format!("{}_case_{}", lbl_prefix, i);
                    let false_lbl = format!("{}_end", true_lbl);

                    stmts.push(Stmt::Jnz(value, true_lbl.clone(), false_lbl.clone()));
                    stmts.push(lbl(&true_lbl));

                    lower_stmts(case_stmts, stmts, ctx);

                    stmts.push(Stmt::Jmp(end_lbl.clone()));
                    stmts.push(lbl(&false_lbl));
                }

                stmts.push(Stmt::Lbl(end_lbl));
            }
            m::Stmt::FuncCall(name, exprs) => {
                let values = lower_exprs_to_values(exprs, stmts, ctx);
                stmts.push(Stmt::FuncCall(name.to_string(), values));
            }
            m::Stmt::Ret(Some(expr)) => {
                let value = lower_expr_to_value(expr, stmts, ctx);
                stmts.push(Stmt::Ret(Some(value)));
            }
            m::Stmt::Ret(None) => stmts.push(Stmt::Ret(None)),
            m::Stmt::VarDecl(name, expr) => {
                let expr = lower_expr(expr, stmts, ctx);
                stmts.push(Stmt::VarDecl(name.to_string(), Scope::Func, expr));
            }
        }

        deref_tmp_refs(stmts, ctx);
    }
}

fn deref_tmp_refs(stmts: &mut Vec<Stmt>, ctx: &mut LoweringCtx) {
    let tmp_refs = ctx.pop_tmp_refs();

    for (tmp_ref_name, var_name, var_type) in tmp_refs {
        let expr = Expr::Load(
            var_type,
            var_type,
            Value::VarRef(tmp_ref_name, Type::L, Scope::Func),
        );
        stmts.push(Stmt::VarDecl(var_name, Scope::Func, expr));
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
        m::Expr::Cmp(op, lhs, rhs) => {
            let var_name = ctx.uniq_name("cmp");
            let expr = Expr::Cmp(
                lower_op(op),
                lower_expr_to_value(lhs, stmts, ctx),
                lower_expr_to_value(rhs, stmts, ctx),
            );

            stmts.push(Stmt::VarDecl(var_name.to_string(), Scope::Func, expr));

            Value::VarRef(var_name.to_string(), Type::W, Scope::Func)
        }
        m::Expr::ConstBool(true) => Value::ConstW(1),
        m::Expr::ConstBool(false) => Value::ConstW(0),
        m::Expr::ConstDouble(d) => Value::ConstD(*d),
        m::Expr::ConstInt32(i) => Value::ConstW(*i),
        m::Expr::ConstInt64(i) => Value::ConstL(*i),
        m::Expr::ConstStr(s) => {
            let name = ctx.name_for_str(s);
            Value::VarRef(name, Type::L, Scope::Global)
        }
        m::Expr::FuncCall(name, r#type, exprs) => {
            let expr = lower_func_call(name, r#type, exprs, stmts, ctx);
            let r#type = expr.r#type();
            let name = ctx.uniq_name("arg");

            stmts.push(Stmt::VarDecl(name.to_string(), Scope::Func, expr));

            Value::VarRef(name, r#type, Scope::Func)
        }
        m::Expr::Not(expr) => {
            let var_name = ctx.uniq_name("not");
            let value = lower_expr_to_value(expr, stmts, ctx);
            let expr = Expr::Sub(Value::ConstW(1), value);

            stmts.push(Stmt::VarDecl(var_name.to_string(), Scope::Func, expr));

            Value::VarRef(var_name.to_string(), Type::W, Scope::Func)
        }
        m::Expr::VarRef(name, r#type, true) => {
            let tmp_ref_name = ctx.uniq_name("ref");
            let r#type = lower_type(r#type);
            let tmp_ref = (tmp_ref_name.to_string(), name.to_string(), r#type);

            ctx.add_tmp_ref(tmp_ref);

            stmts.push(Stmt::VarDecl(
                tmp_ref_name.to_string(),
                Scope::Func,
                Expr::Alloc8(8),
            ));
            stmts.push(Stmt::Store(
                r#type,
                Value::VarRef(name.to_string(), r#type, Scope::Func),
                Value::VarRef(tmp_ref_name.to_string(), Type::L, Scope::Func),
            ));

            Value::VarRef(tmp_ref_name.to_string(), Type::L, Scope::Func)
        }
        m::Expr::VarRef(name, r#type, false) => {
            Value::VarRef(name.to_string(), lower_type(r#type), Scope::Func)
        }
    }
}

fn lower_expr(expr: &m::Expr, stmts: &mut Vec<Stmt>, ctx: &mut LoweringCtx) -> Expr {
    match expr {
        m::Expr::Cmp(op, lhs, rhs) => Expr::Cmp(
            lower_op(op),
            lower_expr_to_value(lhs, stmts, ctx),
            lower_expr_to_value(rhs, stmts, ctx),
        ),
        m::Expr::ConstBool(_)
        | m::Expr::ConstDouble(_)
        | m::Expr::ConstInt32(_)
        | m::Expr::ConstInt64(_)
        | m::Expr::ConstStr(_)
        | m::Expr::VarRef(_, _, _) => {
            let value = lower_expr_to_value(expr, stmts, ctx);
            Expr::Value(value)
        }
        m::Expr::Not(_) => {
            let value = lower_expr_to_value(expr, stmts, ctx);
            Expr::Sub(Value::ConstW(1), value)
        }
        m::Expr::FuncCall(name, r#type, exprs) => lower_func_call(name, r#type, exprs, stmts, ctx),
    }
}

fn lower_func_call(
    name: &str,
    r#type: &m::Type,
    exprs: &[m::Expr],
    stmts: &mut Vec<Stmt>,
    ctx: &mut LoweringCtx,
) -> Expr {
    let values = lower_exprs_to_values(exprs, stmts, ctx);
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
        m::Type::Double => Type::D,
        m::Type::Bool | m::Type::Int32 => Type::W,
        m::Type::Int64 | m::Type::Ptr(_) | m::Type::Str => Type::L,
    }
}

fn lower_opt_type(r#type: &Option<m::Type>) -> Option<Type> {
    r#type.as_ref().map(lower_type)
}

fn lower_op(op: &m::Op) -> Op {
    match op {
        m::Op::Eq => Op::Eq,
        m::Op::Ne => Op::Ne,
    }
}

fn lbl(name: &str) -> Stmt {
    Stmt::Lbl(name.to_string())
}
