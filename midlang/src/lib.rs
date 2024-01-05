pub enum Type {
    Int32,
    Str,
}

pub enum Visibility {
    Public,
    Private,
}

pub enum MidLang<'a> {
    Module(&'a str, &'a [Decl<'a>]),
}

pub enum FuncArg<'a> {
    Named(&'a str, Type),
}

pub enum FuncArgs<'a> {
    Fixed(&'a [FuncArg<'a>]),
    Variadic(FuncArg<'a>, &'a [FuncArg<'a>]),
}

pub enum Decl<'a> {
    FwdDecl(&'a str, Visibility, Type, FuncArgs<'a>),
    FuncDecl(&'a str, Visibility, Type, FuncArgs<'a>, &'a [Stmt<'a>]),
}

pub enum Stmt<'a> {
    Ret(Expr<'a>),
    VarDecl(&'a str, Expr<'a>),
}

pub enum Expr<'a> {
    ConstInt32(i32),
    ConstStr(&'a str),
    FuncCall(&'a str, Type, &'a [Expr<'a>]),
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub mod hello_world {
        use super::*;

        pub const MODULE: MidLang = MidLang::Module(
            "hello_world",
            &[
                Decl::FwdDecl(
                    "puts",
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(&[FuncArg::Named("s", Type::Str)]),
                ),
                Decl::FuncDecl(
                    "main",
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::Fixed(&[]),
                    &[
                        Stmt::VarDecl(
                            "r",
                            Expr::FuncCall("puts", Type::Int32, &[Expr::ConstStr("hello world")]),
                        ),
                        Stmt::Ret(Expr::ConstInt32(0)),
                    ],
                ),
            ],
        );

        #[test]
        fn test_structure() {
            match hello_world::MODULE {
                MidLang::Module("hello_world", &[_, _]) => assert!(true),
                _ => assert!(false),
            };
        }
    }
}
