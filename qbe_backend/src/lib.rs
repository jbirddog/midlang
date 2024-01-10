mod il;
mod lower;
mod lower_lang;
mod lowering_context;

use il::generate_il;
use lower::lower;
use midlang::compiler;
use midlang::middle_lang as m;

pub struct Backend {}

pub fn new() -> Backend {
    Backend {}
}

impl compiler::Backend for Backend {
    fn generate_build_artifacts(&self, midlang: &m::MidLang) -> compiler::BackendResult {
        let lower_lang = lower(midlang);
        Ok(generate_il(&lower_lang)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::fs::read_to_string;
    use std::path::Path;

    use midlang::compiler::Backend as _;

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    fn hello_world() -> TestResult {
        let midlang = m::MidLang::Module(
            "hello_world".to_string(),
            vec![
                m::Decl::FwdDecl(
                    "puts".to_string(),
                    m::Visibility::Public,
                    m::Type::Int32,
                    m::FuncArgs::Fixed(vec![m::FuncArg::Named("s".to_string(), m::Type::Str)]),
                ),
                m::Decl::FuncDecl(
                    "main".to_string(),
                    m::Visibility::Public,
                    m::Type::Int32,
                    m::FuncArgs::Fixed(vec![]),
                    vec![
                        m::Stmt::VarDecl(
                            "r".to_string(),
                            m::Expr::FuncCall(
                                "puts".to_string(),
                                m::Type::Int32,
                                vec![m::Expr::ConstStr("hello world".to_string())],
                            ),
                        ),
                        m::Stmt::Ret(m::Expr::ConstInt32(0)),
                    ],
                ),
            ],
        );

        let ba = new().generate_build_artifacts(&midlang)?;
        assert_eq!(ba.len(), 1);
        assert_eq!(ba[0].0, "hello_world.il");

        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("qbe")
            .join("hello_world.il");
        let expected_il = read_to_string(&path)?;

        assert_eq!(ba[0].1, expected_il);

        Ok(())
    }
}
