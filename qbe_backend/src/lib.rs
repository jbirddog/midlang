use std::path::Path;

use ninja_writer::BuildVariables as _;
use ninja_writer::Ninja;

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
    fn generate_build_artifacts(
        &self,
        midlang: &m::MidLang,
        ninja_writer: &mut Ninja,
    ) -> compiler::BackendResult {
        let lower_lang = lower(midlang);
        let build_artifacts = generate_il(&lower_lang)?;

        configure_ninja_build(&build_artifacts, ninja_writer);

        Ok(build_artifacts)
    }
}

fn configure_ninja_build(build_artifacts: &compiler::BuildArtifacts, ninja_writer: &mut Ninja) {
    let qbe = ninja_writer.rule("qbe", "qbe -o $out $in");
    let cc = ninja_writer.rule("cc", "cc -o $out -c $in");
    let link = ninja_writer.rule("link", "cc -o $out $in");
    let output = "a.out";
    let mut objs = Vec::<String>::with_capacity(build_artifacts.len());

    for (il, _) in build_artifacts {
        let asm = with_ext(il, "s");
        let obj = with_ext(il, "o");

        qbe.build([&asm]).with([&il]);
        cc.build([&obj]).with([&asm]);
        objs.push(obj);
    }

    link.build([&output]).with(&objs);
    ninja_writer.defaults([&output]);
}

fn with_ext(filename: &str, ext: &str) -> String {
    Path::new(filename)
        .with_extension(ext)
        .display()
        .to_string()
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

        let mut ninja_writer = Ninja::new();
        let ba = new().generate_build_artifacts(&midlang, &mut ninja_writer)?;
        assert_eq!(ba.len(), 1);
        assert_eq!(ba[0].0, "hello_world.il");

        let path = Path::new(env!("TEST_CASES_DIR"))
            .join("qbe")
            .join("hello_world.il");
        let expected_il = read_to_string(&path)?;

        assert_eq!(ba[0].1, expected_il);

        let ninja_build = ninja_writer.to_string();
        assert!(ninja_build.contains("hello_world.il"));
        assert!(ninja_build.contains("hello_world.s"));
        assert!(ninja_build.contains("hello_world.o"));
        assert!(ninja_build.contains("a.out"));

        Ok(())
    }
}
