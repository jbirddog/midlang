use midlang::*;

pub fn cmp() -> Vec<Module> {
    vec![Module {
        name: "cmp".to_string(),
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
                            Expr::Cmp(
                                Op::Eq,
                                Box::new(Expr::ConstBool(false)),
                                Box::new(Expr::ConstBool(true)),
                            ),
                            vec![Stmt::FuncCall(
                                "exit".to_string(),
                                vec![Expr::ConstInt32(1)],
                            )],
                        ),
                        (
                            Expr::Cmp(
                                Op::Eq,
                                Box::new(Expr::ConstInt32(12)),
                                Box::new(Expr::ConstInt32(21)),
                            ),
                            vec![Stmt::FuncCall(
                                "exit".to_string(),
                                vec![Expr::ConstInt32(2)],
                            )],
                        ),
                        (
                            Expr::Cmp(
                                Op::Ne,
                                Box::new(Expr::ConstInt64(12)),
                                Box::new(Expr::ConstInt64(12)),
                            ),
                            vec![Stmt::FuncCall(
                                "exit".to_string(),
                                vec![Expr::ConstInt32(3)],
                            )],
                        ),
                    ]),
                    Stmt::FuncCall(
                        "puts".to_string(),
                        vec![Expr::ConstStr("cmp works!".to_string())],
                    ),
                    Stmt::Ret(Some(Expr::ConstInt32(0))),
                ],
            ),
        ],
    }]
}
