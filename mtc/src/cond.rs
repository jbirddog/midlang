use midlang::*;

pub fn cond() -> Vec<Module> {
    vec![Module {
        name: "cond".to_string(),
        decls: vec![
            Decl::FwdDecl(
                "puts".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![("s".to_string(), Type::Str)],
                false,
            ),
            Decl::FwdDecl(
                "exit".to_string(),
                Visibility::Public,
                None,
                vec![("status".to_string(), Type::Int32)],
                false,
            ),
            Decl::FuncDecl(
                "main".to_string(),
                Visibility::Public,
                Some(Type::Int32),
                vec![],
                false,
                vec![
                    Stmt::Cond(vec![
                        (
                            Expr::ConstBool(false),
                            vec![Stmt::Cond(vec![(
                                Expr::ConstBool(true),
                                vec![Stmt::FuncCall(
                                    "exit".to_string(),
                                    vec![Expr::ConstInt32(1)],
                                )],
                            )])],
                        ),
                        (
                            Expr::ConstBool(true),
                            vec![Stmt::VarDecl(
                                "r".to_string(),
                                Expr::FuncCall(
                                    "puts".to_string(),
                                    Type::Int32,
                                    vec![Expr::ConstStr("cond".to_string())],
                                ),
                            )],
                        ),
                    ]),
                    Stmt::Cond(vec![
                        (
                            Expr::ConstBool(true),
                            vec![Stmt::VarDecl(
                                "r".to_string(),
                                Expr::FuncCall(
                                    "puts".to_string(),
                                    Type::Int32,
                                    vec![Expr::ConstStr("works".to_string())],
                                ),
                            )],
                        ),
                        (
                            Expr::ConstBool(true),
                            vec![Stmt::Cond(vec![(
                                Expr::ConstBool(true),
                                vec![Stmt::FuncCall(
                                    "exit".to_string(),
                                    vec![Expr::ConstInt32(1)],
                                )],
                            )])],
                        ),
                    ]),
                    Stmt::FuncCall("puts".to_string(), vec![Expr::ConstStr("ok".to_string())]),
                    Stmt::Ret(Some(Expr::ConstInt32(0))),
                ],
            ),
        ],
    }]
}
