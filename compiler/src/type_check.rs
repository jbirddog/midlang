use std::collections::HashMap;
use std::error::Error;
use std::iter::zip;

use midlang::*;

type FuncSig<'a> = (
    &'a Visibility,
    &'a Option<Type>,
    &'a Vec<FuncArg>,
    &'a Variadic,
);
type FwdDecls<'a> = HashMap<&'a str, FuncSig<'a>>;
type Res<T> = Result<T, Box<dyn Error>>;
type Vars<'a> = HashMap<&'a str, &'a Type>;

pub fn type_check(modules: &[Module]) -> Res<()> {
    for module in modules {
        check_decls(&module.decls)?;
    }

    Ok(())
}

fn check_decls(decls: &[Decl]) -> Res<()> {
    fn variadic_err(name: &str) -> Res<()> {
        Err(format!(
            "Func '{}' requires at least one argument since it is variadic",
            name
        )
        .into())
    }

    let mut fwd_decls = FwdDecls::with_capacity(decls.len());

    for decl in decls {
        match decl {
            Decl::FwdDecl(name, visibility, r#type, args, variadic) => {
                if *variadic && args.is_empty() {
                    return variadic_err(name);
                }

                fwd_decls.insert(name, (visibility, r#type, args, variadic));
            }
            Decl::FuncDecl(name, visibility, r#type, args, variadic, stmts) => {
                if *variadic && args.is_empty() {
                    return variadic_err(name);
                }

                let sig = (visibility, r#type, args, variadic);
                let fwd_sig = fwd_decls.entry(name).or_insert(sig);

                if sig != *fwd_sig {
                    return Err(format!("FwdDecl mismatch for func '{}'", name).into());
                }

                let mut vars = args
                    .iter()
                    .map(|a| (a.0.as_ref(), &a.1))
                    .collect::<HashMap<_, _>>();

                if args.len() != vars.len() {
                    return Err(format!("Args for func '{}' must have unique names", name).into());
                }

                check_stmts(stmts, r#type, &fwd_decls, &mut vars)?;
            }
        }
    }

    Ok(())
}

fn check_stmts<'a>(
    stmts: &'a [Stmt],
    func_type: &Option<Type>,
    fwd_decls: &FwdDecls,
    vars: &'a mut Vars<'a>,
) -> Res<()> {
    fn ret_type_mismatch_err() -> Res<()> {
        Err("Return statment type does not match function type".into())
    }

    for stmt in stmts {
        match stmt {
            Stmt::Cond(cases) => {
                for (expr, stmts) in cases {
                    if *expr.r#type() != Type::Bool {
                        return Err("Cond case expressions must be of type bool".into());
                    }
                    let mut cond_vars = vars.clone();
                    check_stmts(stmts, func_type, fwd_decls, &mut cond_vars)?;
                }
            }
            Stmt::FuncCall(_, exprs) => check_exprs(exprs, fwd_decls, vars)?,
            Stmt::Ret(ret) => match (func_type, ret) {
                (Some(func_type), Some(expr)) if expr.r#type() != func_type => {
                    return ret_type_mismatch_err();
                }
                (Some(_), None) | (None, Some(_)) => {
                    return ret_type_mismatch_err();
                }
                (Some(_), Some(expr)) => check_expr(expr, fwd_decls, vars)?,
                (None, None) => (),
            },
            Stmt::VarDecl(name, expr) => {
                check_expr(expr, fwd_decls, vars)?;
                vars.insert(name, expr.r#type());
            }
        }
    }

    Ok(())
}

fn check_exprs(exprs: &[Expr], fwd_decls: &FwdDecls, vars: &Vars) -> Res<()> {
    for expr in exprs {
        check_expr(expr, fwd_decls, vars)?;
    }

    Ok(())
}

fn check_expr(expr: &Expr, fwd_decls: &FwdDecls, vars: &Vars) -> Res<()> {
    fn func_call_type_err(name: &str) -> Res<()> {
        Err(format!(
            "FuncCall '{}' type does not match forward declaration",
            name
        )
        .into())
    }
    fn param_count_err(name: &str) -> Res<()> {
        Err(format!(
            "FuncCall '{}' parameter count does not match forward declaration",
            name
        )
        .into())
    }

    fn param_type_err(name: &str, i: usize) -> Res<()> {
        Err(format!(
            "FuncCall '{}' parameter {} type does not match forward declaration",
            name, i
        )
        .into())
    }

    match expr {
        Expr::ConstBool(_)
        | Expr::ConstDouble(_)
        | Expr::ConstInt32(_)
        | Expr::ConstInt64(_)
        | Expr::ConstStr(_) => (),
        Expr::VarRef(name, r#type, _) => match vars.get(&name as &str) {
            Some(expr_type) if *expr_type != r#type => {
                return Err(format!("VarRef '{}' type does not match its declaration", name).into())
            }
            Some(_) => (),
            None => return Err(format!("VarRef '{}' does not have a declaration", name).into()),
        },
        Expr::FuncCall(name, call_type, exprs) => {
            match fwd_decls.get(&name as &str) {
                Some((_, None, _, _)) => {
                    return func_call_type_err(name);
                }
                Some((_, Some(fwd_type), _, _)) if call_type != fwd_type => {
                    return func_call_type_err(name);
                }
                Some((_, _, fwd_args, false)) if exprs.len() != fwd_args.len() => {
                    return param_count_err(name);
                }
                Some((_, _, fwd_args, true)) if exprs.len() < fwd_args.len() => {
                    return param_count_err(name);
                }
                Some((_, _, fwd_args, _)) => {
                    for (i, ((_, r#type), expr)) in zip(*fwd_args, exprs).enumerate() {
                        match (r#type, expr) {
                            (Type::Ptr(Some(r#type)), Expr::VarRef(_, expr_type, true)) => {
                                if r#type.as_ref() != expr_type {
                                    return param_type_err(name, i);
                                }
                            }
                            _ => {
                                if r#type != expr.r#type() {
                                    return param_type_err(name, i);
                                }
                            }
                        }
                    }
                }
                None => {
                    return Err(format!(
                        "Calling func '{}' which does not have a forward declaration",
                        name
                    )
                    .into());
                }
            }

            check_exprs(exprs, fwd_decls, vars)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Res<()>;

    #[test]
    fn hello_world() -> TestResult {
        let modules = mtc::hello_world();

        type_check(&modules)?;

        Ok(())
    }

    #[test]
    fn hello_world_cond() -> TestResult {
        let modules = mtc::hello_world_cond();

        type_check(&modules)?;

        Ok(())
    }

    #[test]
    fn nested_func_call() -> Res<()> {
        let modules = mtc::nested_func_call();

        type_check(&modules)?;

        Ok(())
    }

    #[test]
    fn func_call_variadic_params_just_one() -> TestResult {
        let modules = mtc::func_call_variadic_params_just_one();

        type_check(&modules)?;

        Ok(())
    }

    #[test]
    fn func_call_variadic_params_multiple() -> TestResult {
        let modules = mtc::func_call_variadic_params_multiple();

        type_check(&modules)?;

        Ok(())
    }

    #[test]
    fn var_ref() -> TestResult {
        let modules = mtc::var_ref();

        type_check(&modules)?;

        Ok(())
    }

    #[test]
    fn void_main() -> TestResult {
        let modules = mtc::void_main();

        type_check(&modules)?;

        Ok(())
    }

    #[test]
    fn fabs() -> TestResult {
        let modules = mtc::fabs();

        type_check(&modules)?;

        Ok(())
    }

    #[test]
    fn frexp() -> TestResult {
        let modules = mtc::frexp();

        type_check(&modules)?;

        Ok(())
    }

    #[test]
    #[should_panic(expected = "FwdDecl mismatch for func 'main'")]
    fn func_decl_fwd_decl_mismatch() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Str),
                    vec![("s".to_string(), Type::Str)],
                    false,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![Stmt::Ret(Some(Expr::ConstInt32(0)))],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(expected = "FwdDecl mismatch for func 'main'")]
    fn func_decl_fwd_decl_mismatch2() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "main".to_string(),
                    Visibility::Public,
                    None,
                    vec![("s".to_string(), Type::Str)],
                    false,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![Stmt::Ret(Some(Expr::ConstInt32(0)))],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(expected = "Args for func 'main' must have unique names")]
    fn func_decl_non_uniq_arg_names() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![Decl::FuncDecl(
                "main".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![("s".to_string(), Type::Str), ("s".to_string(), Type::Str)],
                false,
                vec![Stmt::Ret(Some(Expr::ConstInt32(0)))],
            )],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(expected = "Calling func 'puts' which does not have a forward declaration")]
    fn func_call_no_fwd_decl() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![Decl::FuncDecl(
                "main".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![],
                false,
                vec![
                    Stmt::VarDecl(
                        "r".to_string(),
                        Expr::FuncCall(
                            "puts".to_string(),
                            Type::Int32,
                            vec![Expr::ConstStr("hello world".to_string())],
                        ),
                    ),
                    Stmt::Ret(Some(Expr::ConstInt32(0))),
                ],
            )],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(expected = "Return statment type does not match function type")]
    fn func_ret_type_mismatch() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![Decl::FuncDecl(
                "main".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![],
                false,
                vec![Stmt::Ret(Some(Expr::ConstStr("hello world".to_string())))],
            )],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(expected = "Return statment type does not match function type")]
    fn func_ret_type_mismatch2() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![Decl::FuncDecl(
                "main".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![],
                false,
                vec![Stmt::Ret(None)],
            )],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(expected = "VarRef 'missing' does not have a declaration")]
    fn var_ref_no_decl() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![Decl::FuncDecl(
                "main".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![],
                false,
                vec![Stmt::Ret(Some(Expr::VarRef(
                    "missing".to_string(),
                    Type::Int32,
                    false,
                )))],
            )],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(expected = "VarRef 'x' type does not match its declaration")]
    fn var_ref_decl_mismatch() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![Decl::FuncDecl(
                "main".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![],
                false,
                vec![
                    Stmt::VarDecl("x".to_string(), Expr::ConstBool(true)),
                    Stmt::Ret(Some(Expr::VarRef("x".to_string(), Type::Int32, false))),
                ],
            )],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(expected = "FuncCall 'puts' parameter count does not match forward declaration")]
    fn func_call_fewer_fixed_params() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "puts".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![("s".to_string(), Type::Str)],
                    false,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![
                        Stmt::VarDecl(
                            "r".to_string(),
                            Expr::FuncCall("puts".to_string(), Type::Int32, vec![]),
                        ),
                        Stmt::Ret(Some(Expr::ConstInt32(0))),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(expected = "FuncCall 'puts' parameter count does not match forward declaration")]
    fn func_call_more_fixed_params() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "puts".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![("s".to_string(), Type::Str)],
                    false,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![
                        Stmt::VarDecl(
                            "r".to_string(),
                            Expr::FuncCall(
                                "puts".to_string(),
                                Type::Int32,
                                vec![
                                    Expr::ConstStr("hello world".to_string()),
                                    Expr::ConstStr("err?".to_string()),
                                ],
                            ),
                        ),
                        Stmt::Ret(Some(Expr::ConstInt32(0))),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "FuncCall 'puts' parameter 0 type does not match forward declaration"
    )]
    fn func_call_fixed_param_type_mismatch() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "puts".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![("s".to_string(), Type::Str)],
                    false,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![
                        Stmt::VarDecl(
                            "r".to_string(),
                            Expr::FuncCall(
                                "puts".to_string(),
                                Type::Int32,
                                vec![Expr::ConstInt32(1)],
                            ),
                        ),
                        Stmt::Ret(Some(Expr::ConstInt32(0))),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "FuncCall 'frexp' parameter 1 type does not match forward declaration"
    )]
    fn func_call_by_ref_type_mismatch() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "frexp".to_string(),
                    Visibility::Public,
                    Some(Type::Double),
                    vec![
                        ("x".to_string(), Type::Double),
                        ("exp".to_string(), Type::Ptr(Some(Box::new(Type::Int32)))),
                    ],
                    false,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![
                        Stmt::VarDecl("bad_exp".to_string(), Expr::ConstBool(false)),
                        Stmt::VarDecl(
                            "r".to_string(),
                            Expr::FuncCall(
                                "frexp".to_string(),
                                Type::Double,
                                vec![
                                    Expr::ConstDouble(2560.0),
                                    Expr::VarRef("bad_exp".to_string(), Type::Bool, true),
                                ],
                            ),
                        ),
                        Stmt::Ret(Some(Expr::ConstInt32(0))),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "FuncCall 'not_ok' parameter 0 type does not match forward declaration"
    )]
    fn nested_func_call_type_mismatch() {
        let modules = [Module {
            name: "hello_world".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "puts".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![("s".to_string(), Type::Str)],
                    false,
                ),
                Decl::FwdDecl(
                    "not_ok".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![("s".to_string(), Type::Str)],
                    false,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![Stmt::Ret(Some(Expr::FuncCall(
                        "not_ok".to_string(),
                        Type::Int32,
                        vec![Expr::FuncCall(
                            "puts".to_string(),
                            Type::Int32,
                            vec![Expr::ConstStr("hello world".to_string())],
                        )],
                    )))],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "FuncCall 'printf' parameter count does not match forward declaration"
    )]
    fn func_call_variadic_params_just_one_too_few() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printf".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![("fmt".to_string(), Type::Str)],
                    true,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![
                        Stmt::VarDecl(
                            "r1".to_string(),
                            Expr::FuncCall("printf".to_string(), Type::Int32, vec![]),
                        ),
                        Stmt::Ret(Some(Expr::ConstInt32(0))),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(expected = "Func 'printf' requires at least one argument since it is variadic")]
    fn fwd_decl_variadic_no_params() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printf".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    true,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![
                        Stmt::VarDecl(
                            "r1".to_string(),
                            Expr::FuncCall("printf".to_string(), Type::Int32, vec![]),
                        ),
                        Stmt::Ret(Some(Expr::ConstInt32(0))),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(expected = "Func 'main' requires at least one argument since it is variadic")]
    fn func_decl_variadic_no_params() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printf".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    true,
                    vec![
                        Stmt::VarDecl(
                            "r1".to_string(),
                            Expr::FuncCall("printf".to_string(), Type::Int32, vec![]),
                        ),
                        Stmt::Ret(Some(Expr::ConstInt32(0))),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "FuncCall 'printf' parameter 0 type does not match forward declaration"
    )]
    fn func_call_variadic_params_just_one_type_mismatch() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printf".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![("fmt".to_string(), Type::Str)],
                    true,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![
                        Stmt::VarDecl(
                            "r1".to_string(),
                            Expr::FuncCall(
                                "printf".to_string(),
                                Type::Int32,
                                vec![Expr::ConstInt32(1)],
                            ),
                        ),
                        Stmt::Ret(Some(Expr::ConstInt32(0))),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "FuncCall 'printnf' parameter count does not match forward declaration"
    )]
    fn func_call_variadic_params_with_many_too_few() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printnf".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![
                        ("fmt".to_string(), Type::Str),
                        ("n".to_string(), Type::Int32),
                    ],
                    true,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![
                        Stmt::VarDecl(
                            "r1".to_string(),
                            Expr::FuncCall(
                                "printnf".to_string(),
                                Type::Int32,
                                vec![Expr::ConstStr("hello world".to_string())],
                            ),
                        ),
                        Stmt::Ret(Some(Expr::ConstInt32(0))),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "FuncCall 'printnf' parameter 1 type does not match forward declaration"
    )]
    fn func_call_variadic_params_with_many_type_mismatch() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printnf".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![
                        ("fmt".to_string(), Type::Str),
                        ("n".to_string(), Type::Int32),
                    ],
                    true,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![
                        Stmt::VarDecl(
                            "r1".to_string(),
                            Expr::FuncCall(
                                "printnf".to_string(),
                                Type::Int32,
                                vec![
                                    Expr::ConstStr("hello world".to_string()),
                                    Expr::ConstStr("hello world".to_string()),
                                ],
                            ),
                        ),
                        Stmt::Ret(Some(Expr::ConstInt32(0))),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    #[should_panic(expected = "Cond case expressions must be of type bool")]
    fn func_cond_case_not_bool() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printnf".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![
                        ("fmt".to_string(), Type::Str),
                        ("n".to_string(), Type::Int32),
                    ],
                    true,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Some(Type::Int32),
                    vec![],
                    false,
                    vec![
                        Stmt::Cond(vec![(
                            Expr::ConstInt32(3),
                            vec![Stmt::VarDecl(
                                "r1".to_string(),
                                Expr::FuncCall(
                                    "printnf".to_string(),
                                    Type::Int32,
                                    vec![
                                        Expr::ConstStr("hello world".to_string()),
                                        Expr::ConstStr("hello world".to_string()),
                                    ],
                                ),
                            )],
                        )]),
                        Stmt::Ret(Some(Expr::ConstInt32(0))),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }
}
