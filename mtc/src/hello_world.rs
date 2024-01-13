use midlang::*;

pub fn hello_world() -> Vec<Module> {
    vec![Module {
        name: "hello_world".to_string(),
        decls: vec![
            Decl::FwdDecl(
                "puts".to_string(),
                Visibility::Public,
                Type::Int32,
                vec![("s".to_string(), Type::Str)],
                false,
            ),
            Decl::FuncDecl(
                "main".to_string(),
                Visibility::Public,
                Type::Int32,
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
                    Stmt::Ret(Expr::ConstInt32(0)),
                ],
            ),
        ],
    }]
}

pub fn hello_world2() -> Vec<Module> {
    vec![
        Module {
            name: "hello_world2".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "say_hello_world".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    vec![],
                    false,
                ),
                Decl::FuncDecl(
                    "main".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    vec![],
                    false,
                    vec![
                        Stmt::VarDecl(
                            "r".to_string(),
                            Expr::FuncCall("say_hello_world".to_string(), Type::Int32, vec![]),
                        ),
                        Stmt::Ret(Expr::ConstInt32(0)),
                    ],
                ),
            ],
        },
        Module {
            name: "hello_world2_sayer".to_string(),
            decls: vec![
                Decl::FwdDecl(
                    "puts".to_string(),
                    Visibility::Public,
                    Type::Int32,
                    vec![("s".to_string(), Type::Str)],
                    false,
                ),
                Decl::FuncDecl(
                    "say_hello_world".to_string(),
                    Visibility::Public,
                    Type::Int32,
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
                        Stmt::Ret(Expr::ConstInt32(0)),
                    ],
                ),
            ],
        },
    ]
}
