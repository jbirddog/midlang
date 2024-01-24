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
