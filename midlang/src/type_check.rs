use std::collections::HashMap;
use std::error::Error;
use std::iter::zip;

use crate::middle_lang::*;

type FuncSig<'a> = (&'a Visibility, &'a Type, &'a FuncArgs);
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
    let mut fwd_decls = FwdDecls::with_capacity(decls.len());

    for decl in decls {
        match decl {
            Decl::FwdDecl(name, visibility, r#type, args) => {
                fwd_decls.insert(name, (visibility, r#type, args));
            }
            Decl::FuncDecl(name, visibility, r#type, args, stmts) => {
                let sig = (visibility, r#type, args);
                let fwd_sig = fwd_decls.entry(name).or_insert(sig);

                if sig != *fwd_sig {
                    return Err(format!("FwdDecl mismatch for func '{}'", name).into());
                }

                let mut vars = vars_from_args(args)?;

                if args.len() != vars.len() {
                    return Err(format!("Args for func '{}' must have unique names", name).into());
                }

                check_stmts(stmts, r#type, &fwd_decls, &mut vars)?;
            }
        }
    }

    Ok(())
}

fn vars_from_args(args: &FuncArgs) -> Res<Vars> {
    let (arg, args) = match args {
        FuncArgs::Fixed(args) => (None, args),
        FuncArgs::Variadic(first, rest) => (Some(first), rest),
    };

    let mut vars = args
        .iter()
        .map(|a| (a.name(), a.r#type()))
        .collect::<HashMap<_, _>>();

    if let Some(arg) = arg {
        vars.insert(arg.name(), arg.r#type());
    }

    Ok(vars)
}

fn check_stmts<'a>(
    stmts: &'a [Stmt],
    func_type: &Type,
    fwd_decls: &FwdDecls,
    vars: &'a mut Vars<'a>,
) -> Res<()> {
    for stmt in stmts {
        match stmt {
            Stmt::Ret(expr) if expr.r#type() != func_type => {
                return Err("Return statment type does not match function type".into());
            }
            Stmt::Ret(expr) => check_expr(expr, fwd_decls, vars)?,
            Stmt::VarDecl(name, expr) => {
                check_expr(expr, fwd_decls, vars)?;
                vars.insert(name, expr.r#type());
            }
        }
    }

    Ok(())
}

fn check_expr(expr: &Expr, fwd_decls: &FwdDecls, _vars: &mut Vars) -> Res<()> {
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
        Expr::ConstInt32(_) | Expr::ConstStr(_) => (),
        Expr::FuncCall(name, call_type, exprs) => {
            match fwd_decls.get(&name as &str) {
                Some((_, fwd_type, _)) if call_type != *fwd_type => {
                    return Err(format!(
                        "FuncCall '{}' type does not match forward declaration",
                        name
                    )
                    .into());
                }
                Some((_, _, FuncArgs::Fixed(fwd_args))) if exprs.len() != fwd_args.len() => {
                    return param_count_err(name);
                }
                Some((_, _, FuncArgs::Fixed(fwd_args))) => {
                    for (i, (arg, expr)) in zip(fwd_args, exprs).enumerate() {
                        if arg.r#type() != expr.r#type() {
                            return param_type_err(name, i);
                        }
                    }
                }
                Some((_, _, FuncArgs::Variadic(_, rest))) if exprs.len() < rest.len() + 1 => {
                    return param_count_err(name);
                }
                Some((_, _, FuncArgs::Variadic(first, rest))) => {
                    if exprs[0].r#type() != first.r#type() {
                        return param_type_err(name, 0);
                    }

                    for (i, (arg, expr)) in
                        zip(rest, exprs.iter().skip(1).take(rest.len())).enumerate()
                    {
                        if arg.r#type() != expr.r#type() {
                            return param_type_err(name, i + 1);
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

            for expr in exprs {
                check_expr(expr, fwd_decls, _vars)?;
            }
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
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "puts".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![FuncArg::Named("s".to_string(), Type::Str)]),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
                    vec![
                        Stmt::VarDecl(
                            "r".to_string(),
                            Expr::FuncCall(
                                "puts".to_string(),
                                Type::Int32,
                                vec![Expr::ConstStr("hello world".to_string())],
                            ),
                        ),
                        Stmt::Ret(Expr::ConstInt32(0)),
                    ],
                ),
            ],
        }];

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
                    Type::Str,
                    FuncArgs::Fixed(vec![FuncArg::Named("s".to_string(), Type::Str)]),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
                    vec![Stmt::Ret(Expr::ConstInt32(0))],
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
                Type::Int32,
                FuncArgs::Fixed(vec![
                    FuncArg::Named("s".to_string(), Type::Str),
                    FuncArg::Named("s".to_string(), Type::Str),
                ]),
                vec![Stmt::Ret(Expr::ConstInt32(0))],
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
                Type::Int32,
                FuncArgs::Fixed(vec![]),
                vec![
                    Stmt::VarDecl(
                        "r".to_string(),
                        Expr::FuncCall(
                            "puts".to_string(),
                            Type::Int32,
                            vec![Expr::ConstStr("hello world".to_string())],
                        ),
                    ),
                    Stmt::Ret(Expr::ConstInt32(0)),
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
                Type::Int32,
                FuncArgs::Fixed(vec![]),
                vec![Stmt::Ret(Expr::ConstStr("hello world".to_string()))],
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
                    Type::Int32,
                    FuncArgs::Fixed(vec![FuncArg::Named("s".to_string(), Type::Str)]),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
                    vec![
                        Stmt::VarDecl(
                            "r".to_string(),
                            Expr::FuncCall("puts".to_string(), Type::Int32, vec![]),
                        ),
                        Stmt::Ret(Expr::ConstInt32(0)),
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
                    Type::Int32,
                    FuncArgs::Fixed(vec![FuncArg::Named("s".to_string(), Type::Str)]),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
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
                        Stmt::Ret(Expr::ConstInt32(0)),
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
                    Type::Int32,
                    FuncArgs::Fixed(vec![FuncArg::Named("s".to_string(), Type::Str)]),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
                    vec![
                        Stmt::VarDecl(
                            "r".to_string(),
                            Expr::FuncCall(
                                "puts".to_string(),
                                Type::Int32,
                                vec![Expr::ConstInt32(1)],
                            ),
                        ),
                        Stmt::Ret(Expr::ConstInt32(0)),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    fn nested_func_call() -> Res<()> {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "puts".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![FuncArg::Named("s".to_string(), Type::Str)]),
                ),
                Decl::FwdDecl(
                    "ok".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![FuncArg::Named("n".to_string(), Type::Int32)]),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
                    vec![Stmt::Ret(Expr::FuncCall(
                        "ok".to_string(),
                        Type::Int32,
                        vec![Expr::FuncCall(
                            "puts".to_string(),
                            Type::Int32,
                            vec![Expr::ConstStr("hello world".to_string())],
                        )],
                    ))],
                ),
            ],
        }];

        type_check(&modules)?;

        Ok(())
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
                    Type::Int32,
                    FuncArgs::Fixed(vec![FuncArg::Named("s".to_string(), Type::Str)]),
                ),
                Decl::FwdDecl(
                    "not_ok".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![FuncArg::Named("s".to_string(), Type::Str)]),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
                    vec![Stmt::Ret(Expr::FuncCall(
                        "not_ok".to_string(),
                        Type::Int32,
                        vec![Expr::FuncCall(
                            "puts".to_string(),
                            Type::Int32,
                            vec![Expr::ConstStr("hello world".to_string())],
                        )],
                    ))],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }

    #[test]
    fn func_call_variadic_params_just_first() -> TestResult {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printf".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Variadic(FuncArg::Named("fmt".to_string(), Type::Str), vec![]),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
                    vec![
                        Stmt::VarDecl(
                            "r1".to_string(),
                            Expr::FuncCall(
                                "printf".to_string(),
                                Type::Int32,
                                vec![Expr::ConstStr("hello world".to_string())],
                            ),
                        ),
                        Stmt::VarDecl(
                            "r2".to_string(),
                            Expr::FuncCall(
                                "printf".to_string(),
                                Type::Int32,
                                vec![
                                    Expr::ConstStr("hello %s".to_string()),
                                    Expr::ConstStr("world".to_string()),
                                ],
                            ),
                        ),
                        Stmt::Ret(Expr::ConstInt32(0)),
                    ],
                ),
            ],
        }];

        type_check(&modules)?;

        Ok(())
    }

    #[test]
    fn func_call_variadic_params_with_rest() -> TestResult {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printnf".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Variadic(
                        FuncArg::Named("fmt".to_string(), Type::Str),
                        vec![FuncArg::Named("n".to_string(), Type::Int32)],
                    ),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
                    vec![
                        Stmt::VarDecl(
                            "r1".to_string(),
                            Expr::FuncCall(
                                "printnf".to_string(),
                                Type::Int32,
                                vec![
                                    Expr::ConstStr("hello world".to_string()),
                                    Expr::ConstInt32(0),
                                ],
                            ),
                        ),
                        Stmt::VarDecl(
                            "r2".to_string(),
                            Expr::FuncCall(
                                "printnf".to_string(),
                                Type::Int32,
                                vec![
                                    Expr::ConstStr("hello %s %d".to_string()),
                                    Expr::ConstInt32(1),
                                    Expr::ConstStr("world".to_string()),
                                    Expr::ConstInt32(11),
                                ],
                            ),
                        ),
                        Stmt::Ret(Expr::ConstInt32(0)),
                    ],
                ),
            ],
        }];

        type_check(&modules)?;

        Ok(())
    }

    #[test]
    #[should_panic(
        expected = "FuncCall 'printf' parameter count does not match forward declaration"
    )]
    fn func_call_variadic_params_just_first_too_few() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printf".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Variadic(FuncArg::Named("fmt".to_string(), Type::Str), vec![]),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
                    vec![
                        Stmt::VarDecl(
                            "r1".to_string(),
                            Expr::FuncCall("printf".to_string(), Type::Int32, vec![]),
                        ),
                        Stmt::Ret(Expr::ConstInt32(0)),
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
    fn func_call_variadic_params_just_first_type_mismatch() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printf".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Variadic(FuncArg::Named("fmt".to_string(), Type::Str), vec![]),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
                    vec![
                        Stmt::VarDecl(
                            "r1".to_string(),
                            Expr::FuncCall(
                                "printf".to_string(),
                                Type::Int32,
                                vec![Expr::ConstInt32(1)],
                            ),
                        ),
                        Stmt::Ret(Expr::ConstInt32(0)),
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
    fn func_call_variadic_params_with_rest_too_few() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printnf".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Variadic(
                        FuncArg::Named("fmt".to_string(), Type::Str),
                        vec![FuncArg::Named("n".to_string(), Type::Int32)],
                    ),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
                    vec![
                        Stmt::VarDecl(
                            "r1".to_string(),
                            Expr::FuncCall(
                                "printnf".to_string(),
                                Type::Int32,
                                vec![Expr::ConstStr("hello world".to_string())],
                            ),
                        ),
                        Stmt::Ret(Expr::ConstInt32(0)),
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
    fn func_call_variadic_params_with_rest_type_mismatch() {
        let modules = [Module {
            name: "".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "printnf".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Variadic(
                        FuncArg::Named("fmt".to_string(), Type::Str),
                        vec![FuncArg::Named("n".to_string(), Type::Int32)],
                    ),
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(vec![]),
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
                        Stmt::Ret(Expr::ConstInt32(0)),
                    ],
                ),
            ],
        }];

        type_check(&modules).unwrap();
    }
}
