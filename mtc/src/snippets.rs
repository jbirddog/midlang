use midlang::*;

pub fn func_call_variadic_params_multiple() -> Vec<Module> {
    vec![Module {
        name: "func_call_variadic_params_multiple".to_string(),
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
                    Stmt::Ret(Some(Expr::ConstInt32(0))),
                ],
            ),
        ],
    }]
}

pub fn nested_func_call() -> Vec<Module> {
    vec![Module {
        name: "nested_func_call".to_string(),
        decls: vec![
            Decl::FwdDecl(
                "puts".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![("s".to_string(), Type::Str)],
                false,
            ),
            Decl::FwdDecl(
                "ok".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![("n".to_string(), Type::Int32)],
                false,
            ),
            Decl::FuncDecl(
                "main".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![],
                false,
                vec![Stmt::Ret(Some(Expr::FuncCall(
                    "ok".to_string(),
                    Type::Int32,
                    vec![Expr::FuncCall(
                        "puts".to_string(),
                        Type::Int32,
                        vec![Expr::ConstStr("hello world".to_string())],
                    )],
                )))],
            ),
        ],
    }]
}

pub fn func_call_variadic_params_just_one() -> Vec<Module> {
    vec![Module {
        name: "func_call_variadic_params_just_one".to_string(),
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
                    Stmt::Ret(Some(Expr::ConstInt32(0))),
                ],
            ),
        ],
    }]
}

pub fn var_ref() -> Vec<Module> {
    vec![Module {
        name: "var_ref".to_string(),
        decls: vec![Decl::FuncDecl(
            "main".to_string(),
            Visibility::Public,
            Some(Type::Int32),
            vec![],
            false,
            vec![
                Stmt::VarDecl("x".to_string(), Expr::ConstInt32(0)),
                Stmt::Ret(Some(Expr::VarRef("x".to_string(), Type::Int32, false))),
            ],
        )],
    }]
}

pub fn void_main() -> Vec<Module> {
    vec![Module {
        name: "void_main".to_string(),
        decls: vec![Decl::FuncDecl(
            "main".to_string(),
            Visibility::Public,
            None,
            vec![],
            false,
            vec![Stmt::Ret(None)],
        )],
    }]
}
