use midlang::*;

pub fn fabs() -> Vec<Module> {
    vec![Module {
        name: "fabs".to_string(),
        decls: vec![
            Decl::FwdDecl(
                "printf".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![("fmt".to_string(), Type::Str)],
                true,
            ),
            Decl::FwdDecl(
                "fabs".to_string(),
                Visibility::Public,
                Some(Type::Double),
                vec![("x".to_string(), Type::Double)],
                false,
            ),
            Decl::FuncDecl(
                "main".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![],
                false,
                vec![
                    Stmt::FuncCall(
                        "printf".to_string(),
                        vec![
                            Expr::ConstStr("The fabs of -1.23 is %f\n".to_string()),
                            Expr::FuncCall(
                                "fabs".to_string(),
                                Type::Double,
                                vec![Expr::ConstDouble(-1.23)],
                            ),
                        ],
                    ),
                    Stmt::Ret(Some(Expr::ConstInt32(0))),
                ],
            ),
        ],
    }]
}

pub fn frexp() -> Vec<Module> {
    vec![Module {
        name: "frexp".to_string(),
        decls: vec![
            Decl::FwdDecl(
                "printf".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![("fmt".to_string(), Type::Str)],
                true,
            ),
            Decl::FwdDecl(
                "frexp".to_string(),
                Visibility::Public,
                Some(Type::Double),
                vec![
                    ("x".to_string(), Type::Double),
                    ("exp".to_string(), Type::Ptr(Some(Box::new(Type::Int32)))),
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
                    Stmt::VarDecl("exp".to_string(), Expr::ConstInt32(0)),
                    Stmt::FuncCall(
                        "frexp".to_string(),
                        vec![
                            Expr::ConstDouble(2560.0),
                            Expr::VarRef("exp".to_string(), Type::Int32, true),
                        ],
                    ),
                    Stmt::FuncCall(
                        "printf".to_string(),
                        vec![
                            Expr::ConstStr("frexp(2560.0, &e); e = %d\n".to_string()),
                            Expr::VarRef("exp".to_string(), Type::Int32, false),
                        ],
                    ),
                    Stmt::Ret(Some(Expr::ConstInt32(0))),
                ],
            ),
        ],
    }]
}
