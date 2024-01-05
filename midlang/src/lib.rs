pub enum Type {
    Int32,
    Str,
}

pub enum Visibility {
    Public,
    Private,
}

pub struct FuncArg<'a> {
    name: &'a str,
    r#type: Type,
}

pub enum FuncArgs<'a> {
    None,
    Fixed(&'a [FuncArg<'a>]),
    Variadic(FuncArg<'a>, &'a [FuncArg<'a>]),
}

pub struct Module<'a> {
    name: &'a str,
    decls: &'a [Decl<'a>],
}

pub enum Decl<'a> {
    Extern(&'a str, Type, FuncArgs<'a>),
    FwdDecl(&'a str, Visibility, Type, FuncArgs<'a>),
    FuncDecl(&'a str, Visibility, Type, FuncArgs<'a>, &'a [Stmt]),
}

pub enum Stmt {
    Ret(Expr),
}

pub enum Expr {
    Int32(i32),
}

#[cfg(test)]
pub mod test {
    use super::*;

    pub mod hello_world {
        use super::*;

        pub const MODULE: Module = Module {
            name: "hello_world",
            decls: &[
                Decl::Extern(
                    "puts",
                    Type::Int32,
                    FuncArgs::Fixed(&[FuncArg {
                        name: "s",
                        r#type: Type::Str,
                    }]),
                ),
                Decl::FuncDecl(
                    "main",
                    Visibility::Public,
                    Type::Int32,
                    FuncArgs::None,
                    &[Stmt::Ret(Expr::Int32(0))],
                ),
            ],
        };

        #[test]
        fn test_structure() {
            let m = hello_world::MODULE;
            assert_eq!(m.name, "hello_world");
            assert_eq!(m.decls.len(), 2);
        }
    }
}
